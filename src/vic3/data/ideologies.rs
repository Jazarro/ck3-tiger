use crate::block::validator::Validator;
use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::helpers::stringify_choices;
use crate::item::Item;
use crate::report::{warn, ErrorKey};
use crate::scopes::Scopes;
use crate::scriptvalue::validate_scriptvalue;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::{validate_trigger, STANCES};

#[derive(Clone, Debug)]
pub struct Ideology {}

impl Ideology {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Ideology, key, block, Box::new(Self {}));
    }
}

impl DbKind for Ideology {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        let loca = format!("{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_item("icon", Item::File);
        vd.field_bool("show_in_list");

        vd.field_bool("character_ideology");
        if block.field_value_is("character_ideology", "yes") {
            vd.field_validated_key_block("possible", |key, block, data| {
                let mut sc = ScopeContext::new(Scopes::Character, key);
                sc.define_name("interest_group", Scopes::InterestGroup, key);
                validate_trigger(block, data, &mut sc, Tooltipped::No);
            });
            vd.field_validated_key("leader_weight", |key, bv, data| {
                let mut sc = ScopeContext::new(Scopes::Character, key);
                sc.define_name("interest_group", Scopes::InterestGroup, key);
                validate_scriptvalue(bv, data, &mut sc);
            });
            vd.ban_field("priority", || "character_ideology = no");
        } else {
            vd.ban_field("possible", || "character_ideology = yes");
            vd.ban_field("leader_weight", || "character_ideology = yes");
            vd.field_numeric("priority");
        }

        vd.unknown_block_fields(|key, block| {
            data.verify_exists(Item::LawGroup, key);
            let mut vd = Validator::new(block, data);
            vd.unknown_value_fields(|key, value| {
                data.verify_exists(Item::LawType, key);
                if !STANCES.contains(&value.as_str()) {
                    let msg = format!("expected one of {}", stringify_choices(STANCES));
                    warn(ErrorKey::Choice).msg(msg).loc(value).push();
                }
            });
        });
    }
}
