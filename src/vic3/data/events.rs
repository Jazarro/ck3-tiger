use std::path::{Path, PathBuf};
use std::str::FromStr;

use fnv::FnvHashMap;

use crate::block::validator::Validator;
use crate::block::{Block, Field, BV};
use crate::context::ScopeContext;
use crate::desc::validate_desc;
use crate::effect::{validate_effect, validate_effect_internal};
use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::helpers::dup_error;
use crate::item::Item;
use crate::pdxfile::PdxFile;
use crate::report::{error, error_info, warn_info, ErrorKey};
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;
use crate::validate::{validate_ai_chance, validate_duration, ListType};

#[derive(Clone, Debug, Default)]
pub struct Events {
    events: FnvHashMap<(String, u16), Event>,
    namespaces: FnvHashMap<String, Token>,
}

impl Events {
    fn load_event(&mut self, key: Token, block: Block) {
        if let Some((key_a, key_b)) = key.as_str().split_once('.') {
            if let Ok(id) = u16::from_str(key_b) {
                if let Some(other) = self.get_event(key.as_str()) {
                    dup_error(&key, &other.key, "event");
                }
                self.events.insert((key_a.to_string(), id), Event::new(key, block));
                return;
            }
        }
        warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of up to 4 digits.");
    }

    pub fn get_event(&self, key: &str) -> Option<&Event> {
        if let Some((namespace, id)) = key.split_once('.') {
            if let Ok(id) = u16::from_str(id) {
                return self.events.get(&(namespace.to_string(), id));
            }
        }
        None
    }

    pub fn check_scope(&self, token: &Token, sc: &mut ScopeContext) {
        if let Some(event) = self.get_event(token.as_str()) {
            sc.expect(event.expects_scope, token);
        }
    }

    pub fn namespace_exists(&self, key: &str) -> bool {
        self.namespaces.contains_key(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        if let Some((namespace, id)) = key.split_once('.') {
            if let Ok(id) = u16::from_str(id) {
                if self.events.contains_key(&(namespace.to_string(), id)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn validate(&self, data: &Everything) {
        for item in self.events.values() {
            item.validate(data);
        }
    }
}

impl FileHandler<Block> for Events {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("events")
    }

    fn load_file(&self, entry: &FileEntry, fullpath: &Path) -> Option<Block> {
        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return None;
        }

        PdxFile::read(entry, fullpath)
    }

    fn handle_file(&mut self, _entry: &FileEntry, mut block: Block) {
        for item in block.drain() {
            if let Some(Field(key, _, bv)) = item.expect_into_field() {
                if key.is("namespace") {
                    if let Some(value) = bv.expect_into_value() {
                        self.namespaces.insert(value.to_string(), value);
                    }
                } else if let Some(block) = bv.into_block() {
                    self.load_event(key, block);
                } else {
                    let msg = "unknown setting in event files";
                    error(key, ErrorKey::UnknownField, msg);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    key: Token,
    block: Block,
    expects_scope: Scopes,
    expects_from_token: Token,
}

const EVENT_TYPES: &[&str] = &["country_event", "state_event"];

impl Event {
    pub fn new(key: Token, block: Block) -> Self {
        let mut expects_scope = Scopes::Country;
        let mut expects_from_token = key.clone();

        if let Some(event_type) = block.get_field_value("type") {
            match event_type.as_str() {
                "character_event" => {
                    expects_scope = Scopes::Character;
                    expects_from_token = event_type.clone();
                }
                "country_event" => expects_from_token = event_type.clone(),
                "state_event" => {
                    expects_scope = Scopes::State;
                    expects_from_token = event_type.clone();
                }
                _ => (),
            }
        }

        Self { key, block, expects_scope, expects_from_token }
    }

    pub fn validate(&self, data: &Everything) {
        let mut vd = Validator::new(&self.block, data);

        let mut tooltipped_immediate = Tooltipped::Past;
        let mut tooltipped = Tooltipped::Yes;
        if let Some((namespace, _)) = self.key.as_str().split_once('.') {
            if !data.item_exists(Item::EventNamespace, namespace) {
                let msg = format!("event file should start with `namespace = {namespace}`");
                let info = "otherwise the event won't be found in-game";
                error_info(&self.key, ErrorKey::EventNamespace, &msg, info);
            }
        }

        vd.field_choice("type", EVENT_TYPES);

        let mut sc = ScopeContext::new(self.expects_scope, &self.expects_from_token);
        sc.set_strict_scopes(false);

        vd.field_bool("hidden");
        let hidden = self.block.field_value_is("hidden", "yes");
        if hidden {
            tooltipped_immediate = Tooltipped::No;
            tooltipped = Tooltipped::No;
        }

        vd.field_item("dlc", Item::Dlc);

        vd.field_validated_block("event_image", |block, data| {
            let mut vd = Validator::new(block, data);
            if let Some(token) = vd.field_value("video") {
                if token.as_str().contains('/') {
                    data.verify_exists(Item::File, token);
                } else {
                    data.verify_exists(Item::MediaAlias, token);
                }
            }
            vd.field_item("texture", Item::File);
        });

        vd.field_value("gui_window"); // TODO

        vd.field_item("on_created_soundeffect", Item::Sound);
        vd.field_item("on_opened_soundeffect", Item::Sound);
        vd.field_item("icon", Item::File);

        vd.field_integer("duration");

        vd.field_validated_block("trigger", |block, data| {
            validate_trigger(block, data, &mut sc, Tooltipped::No);
        });
        vd.field_validated_block("cancellation_trigger", |block, data| {
            validate_trigger(block, data, &mut sc, Tooltipped::No);
        });
        vd.field_validated_block("immediate", |block, data| {
            validate_effect(block, data, &mut sc, tooltipped_immediate);
        });

        vd.field_validated_sc("title", &mut sc, validate_desc);
        vd.field_validated_sc("desc", &mut sc, validate_desc);
        vd.field_validated_sc("flavor", &mut sc, validate_desc);
        vd.field_validated_block_sc("cooldown", &mut sc, validate_duration);

        vd.field_target(
            "placement",
            &mut sc,
            Scopes::Country | Scopes::State | Scopes::StateRegion,
        );
        vd.field_target("left_icon", &mut sc, Scopes::Character);
        vd.field_target("right_icon", &mut sc, Scopes::Character);
        vd.field_target("minor_left_icon", &mut sc, Scopes::Country);
        vd.field_target("minor_right_icon", &mut sc, Scopes::Country);

        if !hidden {
            vd.req_field("option");
        }
        vd.field_validated_blocks("option", |block, data| {
            validate_event_option(block, data, &mut sc, tooltipped);
        });
    }
}

fn validate_event_option(
    block: &Block,
    data: &Everything,
    sc: &mut ScopeContext,
    tooltipped: Tooltipped,
) {
    // TODO: warn if they use desc, first_valid, random_valid, or triggered_desc directly
    // in the name or tooltip.

    let mut vd = Validator::new(block, data);
    vd.field_validated_bvs("name", |bv, data| match bv {
        BV::Value(t) => {
            data.localization.verify_exists(t);
        }
        BV::Block(b) => {
            let mut vd = Validator::new(b, data);
            vd.req_field("text");
            vd.field_validated_block("trigger", |block, data| {
                validate_trigger(block, data, sc, Tooltipped::No);
            });
            vd.field_validated_sc("text", sc, validate_desc);
        }
    });

    vd.field_validated_block("trigger", |b, data| {
        validate_trigger(b, data, sc, Tooltipped::No);
    });

    vd.field_bool("default_option");
    vd.field_bool("highlighted_option");
    vd.field_validated_sc("ai_chance", sc, validate_ai_chance);
    validate_effect_internal("option", ListType::None, block, data, sc, vd, tooltipped);
}
