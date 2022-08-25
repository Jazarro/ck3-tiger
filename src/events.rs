use fnv::FnvHashMap;
use std::path::{Path, PathBuf};

use crate::block::{Block, BlockOrValue, DefinitionItem, Token};
use crate::desc::verify_desc_locas;
use crate::errorkey::ErrorKey;
use crate::errors::{error, error_info, info, warn, warn_info, will_log, LogPauseRaii};
use crate::everything::FileHandler;
use crate::fileset::{FileEntry, FileKind};
use crate::localization::Localization;
use crate::pdxfile::PdxFile;

#[derive(Clone, Debug, Default)]
pub struct Events {
    events: FnvHashMap<String, Event>,
    scripted_triggers: FnvHashMap<String, ScriptedTrigger>,
    scripted_effects: FnvHashMap<String, ScriptedEffect>,

    // These events are known to exist, so don't warn abour them not being found,
    // but they had errors on validation.
    error_events: FnvHashMap<String, Token>,
}

impl Events {
    pub fn load_event(&mut self, key: Token, block: &Block) {
        if let Some(other) = self.events.get(key.as_str()) {
            if will_log(&key, ErrorKey::Duplicate) {
                error(
                    &key,
                    ErrorKey::Duplicate,
                    "event redefines an existing event",
                );
                info(&other.key, ErrorKey::Duplicate, "the other event is here");
            }
        }
        self.events
            .insert(key.to_string(), Event::new(key, block.clone()));
    }

    pub fn load_scripted_trigger(&mut self, _key: Token, _block: &Block) {}
    pub fn load_scripted_effect(&mut self, _key: Token, _block: &Block) {}

    pub fn check_have_localizations(&self, locs: &Localization) {
        for event in self.events.values() {
            event.check_have_localizations(locs);
        }
    }
}

impl FileHandler for Events {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("events")
    }

    fn config(&mut self, _config: &Block) {}

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        #[derive(Copy, Clone)]
        enum Expecting {
            Event,
            ScriptedTrigger,
            ScriptedEffect,
        }

        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let _pause = LogPauseRaii::new(entry.kind() != FileKind::ModFile);

        let block = match PdxFile::read(entry.path(), entry.kind(), fullpath) {
            Ok(block) => block,
            Err(e) => {
                error_info(
                    entry,
                    ErrorKey::ReadError,
                    "could not read file",
                    &format!("{:#}", e),
                );
                return;
            }
        };

        let mut namespaces = Vec::new();
        let mut expecting = Expecting::Event;

        for def in block.iter_definitions_warn() {
            match def {
                DefinitionItem::Assignment(key, value) if key.is("namespace") => {
                    namespaces.push(value.as_str());
                }
                DefinitionItem::Assignment(key, _)
                    if key.is("scripted_trigger") || key.is("scripted_effect") =>
                {
                    error(
                        key,
                        ErrorKey::Validation,
                        &format!("`{}` should be used without `=`", key),
                    );
                }
                DefinitionItem::Assignment(key, _) => error(
                    key,
                    ErrorKey::Validation,
                    "unknown setting in event files, expected only `namespace`",
                ),
                DefinitionItem::Keyword(key)
                    if matches!(expecting, Expecting::Event) && key.is("scripted_trigger") =>
                {
                    expecting = Expecting::ScriptedTrigger;
                }
                DefinitionItem::Keyword(key)
                    if matches!(expecting, Expecting::Event) && key.is("scripted_effect") =>
                {
                    expecting = Expecting::ScriptedEffect;
                }
                DefinitionItem::Keyword(key) => error_info(
                    key,
                    ErrorKey::Validation,
                    "unexpected token",
                    "Did you forget an = ?",
                ),
                DefinitionItem::Definition(key, b) if key.is("namespace") => {
                    error(
                        b,
                        ErrorKey::EventNamespace,
                        "expected namespace to have a simple string value",
                    );
                }
                DefinitionItem::Definition(key, b) => match expecting {
                    Expecting::ScriptedTrigger => {
                        self.load_scripted_trigger(key.clone(), b);
                        expecting = Expecting::Event;
                    }
                    Expecting::ScriptedEffect => {
                        self.load_scripted_effect(key.clone(), b);
                        expecting = Expecting::Event;
                    }
                    Expecting::Event => {
                        let mut namespace_ok = false;
                        if namespaces.is_empty() {
                            error(
                                key,
                                ErrorKey::EventNamespace,
                                "Event files must start with a namespace declaration",
                            );
                        } else if let Some((key_a, key_b)) = key.as_str().split_once('.') {
                            if key_b.chars().all(|c| c.is_ascii_digit()) {
                                if namespaces.contains(&key_a) {
                                    namespace_ok = true;
                                } else {
                                    warn_info(key, ErrorKey::EventNamespace, "Event name should start with namespace", "If the event doesn't match its namespace, the game can't properly find the event when triggering it.");
                                }
                            } else {
                                warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
                            }
                        } else {
                            warn_info(key, ErrorKey::EventNamespace, "Event names should be in the form NAMESPACE.NUMBER", "where NAMESPACE is the namespace declared at the top of the file, and NUMBER is a series of digits.");
                        }

                        if namespace_ok {
                            self.load_event(key.clone(), b);
                        } else {
                            self.error_events.insert(key.to_string(), key.clone());
                        }
                    }
                },
            }
        }
    }

    fn finalize(&mut self) {}
}

#[derive(Clone, Debug)]
pub struct Event {
    key: Token,
    block: Block,
}

impl Event {
    pub fn new(key: Token, block: Block) -> Self {
        Event { key, block }
    }

    pub fn check_have_localizations(&self, locas: &Localization) {
        if let Some(title) = self.block.get_field("title") {
            verify_desc_locas(title, locas, "event title");
        }
        if let Some(desc) = self.block.get_field("desc") {
            verify_desc_locas(desc, locas, "event description");
        }
        if let Some(opening) = self.block.get_field("opening") {
            verify_desc_locas(opening, locas, "letter event opening");
        }
        for option in self.block.get_field_blocks("option") {
            if let Some(name) = option.get_field("name") {
                match name {
                    BlockOrValue::Token(t) => {
                        locas.verify_have_key(t.as_str(), t, "event option name");
                    }
                    BlockOrValue::Block(b) => {
                        if let Some(text) = b.get_field("text") {
                            verify_desc_locas(text, locas, "event option name");
                        } else {
                            warn(b, ErrorKey::Validation, "event option name with no text");
                        }
                    }
                }
            }
            if let Some(tooltip) = option.get_field("custom_tooltip") {
                match tooltip {
                    BlockOrValue::Token(t) => {
                        locas.verify_have_key(t.as_str(), t, "event option tooltip");
                    }
                    BlockOrValue::Block(b) => {
                        if let Some(text) = b.get_field("text") {
                            verify_desc_locas(text, locas, "event option tooltip");
                        } else {
                            warn(b, ErrorKey::Validation, "event option tooltip with no text");
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScriptedTrigger {}

#[derive(Clone, Debug)]
pub struct ScriptedEffect {}