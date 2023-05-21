use fnv::FnvHashMap;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use crate::block::validator::Validator;
use crate::block::{Block, BlockOrValue};
use crate::context::ScopeContext;
use crate::errorkey::ErrorKey;
use crate::errors::{error, warn, warn_info};
use crate::everything::Everything;
use crate::fileset::{FileEntry, FileHandler};
use crate::helpers::dup_error;
use crate::item::Item;
use crate::pdxfile::PdxFile;
use crate::scopes::{scope_iterator, scope_prefix, scope_to_scope, Scopes};
use crate::token::{Loc, Token};
use crate::trigger::{validate_normal_trigger, validate_target};
use crate::validate::{
    validate_inside_iterator, validate_iterator_fields, validate_prefix_reference, ListType,
};

#[derive(Clone, Debug, Default)]
pub struct ScriptValues {
    scriptvalues: FnvHashMap<String, ScriptValue>,
}

impl ScriptValues {
    fn load_item(&mut self, key: &Token, bv: &BlockOrValue) {
        if let Some(other) = self.scriptvalues.get(key.as_str()) {
            if other.key.loc.kind >= key.loc.kind {
                dup_error(key, &other.key, "script value");
            }
        }
        self.scriptvalues
            .insert(key.to_string(), ScriptValue::new(key.clone(), bv.clone()));
    }

    pub fn exists(&self, key: &str) -> bool {
        self.scriptvalues.contains_key(key)
    }

    pub fn validate(&self, data: &Everything) {
        for item in self.scriptvalues.values() {
            item.validate(data);
        }
    }

    pub fn validate_call(&self, key: &Token, data: &Everything, sc: &mut ScopeContext) {
        if let Some(item) = self.scriptvalues.get(key.as_str()) {
            item.validate_call(key, data, sc);
        }
    }
}

impl FileHandler for ScriptValues {
    fn subpath(&self) -> PathBuf {
        PathBuf::from("common/script_values")
    }

    fn handle_file(&mut self, entry: &FileEntry, fullpath: &Path) {
        if !entry.filename().to_string_lossy().ends_with(".txt") {
            return;
        }

        let Some(block) = PdxFile::read(entry, fullpath) else { return };
        for (key, bv) in block.iter_bv_definitions_warn() {
            self.load_item(key, bv);
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScriptValue {
    key: Token,
    bv: BlockOrValue,
    cache: RefCell<FnvHashMap<Loc, ScopeContext>>,
}

impl ScriptValue {
    pub fn new(key: Token, bv: BlockOrValue) -> Self {
        Self {
            key,
            bv,
            cache: RefCell::new(FnvHashMap::default()),
        }
    }

    fn validate_inner(mut vd: Validator, data: &Everything, sc: &mut ScopeContext) {
        vd.field_value_item("desc", Item::Localization);
        vd.field_value_item("format", Item::Localization);
        // save_temporary_scope_as is now allowed in script values
        vd.field_value("save_temporary_scope_as");
        if let Some(block) = vd.field_block("save_temporary_opinion_value_as") {
            warn_info(block, ErrorKey::Validation, "`save_temporary_opinion_value_as` does not work in script value", "but you can put it in an `if = { limit = { save_temporary_opinion_value_as = ... } }` block inside a script value");
        }
        vd.field_validated("value", |bv, data| {
            Self::validate_bv(bv, data, sc);
        });
        vd.warn_past_known(
            "value",
            "Setting value here will overwrite the previous calculations",
        );
        vd.field_validated_bvs_sc("add", sc, Self::validate_bv);
        vd.field_validated_bvs_sc("subtract", sc, Self::validate_bv);
        vd.field_validated_bvs_sc("multiply", sc, Self::validate_bv);
        // TODO: warn if not sure that divide by zero is impossible?
        vd.field_validated_bvs_sc("divide", sc, Self::validate_bv);
        vd.field_validated_bvs_sc("modulo", sc, Self::validate_bv);
        vd.field_validated_bvs_sc("min", sc, Self::validate_bv);
        vd.field_validated_bvs_sc("max", sc, Self::validate_bv);
        vd.field_bool("round");
        vd.field_bool("ceiling");
        vd.field_bool("floor");
        vd.field_validated_blocks_sc("fixed_range", sc, Self::validate_minmax_range);
        vd.field_validated_blocks_sc("integer_range", sc, Self::validate_minmax_range);
        // TODO: check that these actually follow each other
        vd.field_validated_blocks_sc("if", sc, Self::validate_if);
        vd.field_validated_blocks_sc("else_if", sc, Self::validate_if);
        vd.field_validated_blocks_sc("else", sc, Self::validate_else);

        'outer: for (key, bv) in vd.unknown_keys() {
            if let Some(token) = bv.get_value() {
                error(token, ErrorKey::Validation, "expected block, found value");
                continue;
            }
            let block = bv.get_block().unwrap();

            if let Some((it_type, it_name)) = key.split_once('_') {
                if it_type.is("every")
                    || it_type.is("ordered")
                    || it_type.is("random")
                    || it_type.is("any")
                {
                    if let Some((inscopes, outscope)) = scope_iterator(&it_name, data) {
                        if it_type.is("any") {
                            let msg = "cannot use `any_` iterators in a script value";
                            error(key, ErrorKey::Validation, msg);
                        }
                        sc.expect(inscopes, key);
                        sc.open_scope(outscope, key.clone());
                        Self::validate_iterator(&it_type, &it_name, block, data, sc);
                        sc.close();
                        continue;
                    }
                }
            }

            let mut first = true;
            sc.open_builder();
            for mut part in key.split('.') {
                if let Some((new_part, arg)) = part.split_once('(') {
                    if let Some((arg, _)) = arg.split_once(')') {
                        let arg = arg.trim();
                        if new_part.is("vassal_contract_obligation_level_score") {
                            validate_target(&arg, data, sc, Scopes::VassalContract);
                        } else if new_part.is("squared_distance") {
                            validate_target(&arg, data, sc, Scopes::Province);
                        } else {
                            warn(arg, ErrorKey::Validation, "unexpected argument");
                        }
                        part = new_part;
                    }
                }

                if let Some((prefix, mut arg)) = part.split_once(':') {
                    if prefix.is("event_id") {
                        arg = key.split_once(':').unwrap().1;
                    }
                    if let Some((inscopes, outscope)) = scope_prefix(prefix.as_str()) {
                        if inscopes == Scopes::None && !first {
                            let msg = format!("`{prefix}:` makes no sense except as first part");
                            warn(&part, ErrorKey::Validation, &msg);
                        }
                        sc.expect(inscopes, &prefix);
                        validate_prefix_reference(&prefix, &arg, data);
                        sc.replace(outscope, part);
                        if prefix.is("event_id") {
                            break; // force last part
                        }
                    } else {
                        let msg = format!("unknown prefix `{prefix}:`");
                        error(part, ErrorKey::Validation, &msg);
                        sc.close();
                        continue 'outer;
                    }
                } else if part.lowercase_is("root")
                    || part.lowercase_is("prev")
                    || part.lowercase_is("this")
                {
                    if !first {
                        let msg = format!("`{part}` makes no sense except as first part");
                        warn(&part, ErrorKey::Validation, &msg);
                    }
                    if part.lowercase_is("root") {
                        sc.replace_root();
                    } else if part.lowercase_is("prev") {
                        sc.replace_prev(&part);
                    } else {
                        sc.replace_this();
                    }
                } else if let Some((inscopes, outscope)) = scope_to_scope(&part) {
                    if inscopes == Scopes::None && !first {
                        let msg = format!("`{part}` makes no sense except as first part");
                        warn(&part, ErrorKey::Validation, &msg);
                    }
                    sc.expect(inscopes, &part);
                    sc.replace(outscope, part);
                } else {
                    // TODO: warn if trying to use iterator here
                    let msg = format!("unknown token `{part}`");
                    error(part, ErrorKey::Validation, &msg);
                    sc.close();
                    continue 'outer;
                }
                first = false;
            }
            Self::validate_block(block, data, sc);
            sc.close();
        }
    }

    fn validate_iterator(
        it_type: &Token,
        it_name: &Token,
        block: &Block,
        data: &Everything,
        sc: &mut ScopeContext,
    ) {
        let mut vd = Validator::new(block, data);
        vd.field_validated_block("limit", |block, data| {
            validate_normal_trigger(block, data, sc, false);
        });

        let ltype = ListType::try_from(it_type.as_str()).unwrap();
        let mut tooltipped = false;
        validate_iterator_fields("", ltype, sc, &mut vd, &mut tooltipped);

        validate_inside_iterator(
            it_name.as_str(),
            it_type.as_str(),
            block,
            data,
            sc,
            &mut vd,
            false,
        );

        Self::validate_inner(vd, data, sc);
    }

    fn validate_minmax_range(block: &Block, data: &Everything, sc: &mut ScopeContext) {
        let mut vd = Validator::new(block, data);
        vd.req_field("min");
        vd.req_field("max");
        vd.field_validated_bvs("min", |bv, data| {
            Self::validate_bv(bv, data, sc);
        });
        vd.field_validated_bvs("max", |bv, data| {
            Self::validate_bv(bv, data, sc);
        });
    }

    fn validate_if(block: &Block, data: &Everything, sc: &mut ScopeContext) {
        let mut vd = Validator::new(block, data);
        vd.req_field_warn("limit");
        vd.field_validated_block("limit", |block, data| {
            validate_normal_trigger(block, data, sc, false);
        });
        Self::validate_inner(vd, data, sc);
    }

    fn validate_else(block: &Block, data: &Everything, sc: &mut ScopeContext) {
        let mut vd = Validator::new(block, data);
        vd.field_validated_block("limit", |block, data| {
            validate_normal_trigger(block, data, sc, false);
        });
        Self::validate_inner(vd, data, sc);
    }

    fn validate_block(block: &Block, data: &Everything, sc: &mut ScopeContext) {
        let vd = Validator::new(block, data);
        Self::validate_inner(vd, data, sc);
    }

    pub fn validate_bv(bv: &BlockOrValue, data: &Everything, sc: &mut ScopeContext) {
        match bv {
            BlockOrValue::Token(t) => validate_target(t, data, sc, Scopes::Value | Scopes::Bool),
            BlockOrValue::Block(b) => {
                let mut vd = Validator::new(b, data);
                if let Some((None, _, _)) = b.iter_items().next() {
                    // It's a range like { 1 5 }
                    let vec = vd.values();
                    if vec.len() == 2 {
                        for v in vec {
                            validate_target(v, data, sc, Scopes::Value | Scopes::Bool);
                        }
                    } else {
                        warn(b, ErrorKey::Validation, "invalid script value range");
                    }
                } else {
                    Self::validate_inner(vd, data, sc);
                }
            }
        }
    }

    pub fn cached_compat(&self, key: &Token, sc: &mut ScopeContext) -> bool {
        if let Some(our_sc) = self.cache.borrow().get(&key.loc) {
            sc.expect_compatibility(our_sc, key);
            true
        } else {
            false
        }
    }

    pub fn validate(&self, data: &Everything) {
        // For some reason, script values can be set to bools as well
        if let Some(token) = self.bv.get_value() {
            if token.is("yes") || token.is("no") {
                return;
            }
        }
        let mut sc = ScopeContext::new_unrooted(Scopes::all(), self.key.clone());
        self.validate_call(&self.key, data, &mut sc);
    }

    pub fn validate_call(&self, key: &Token, data: &Everything, sc: &mut ScopeContext) {
        if !self.cached_compat(key, sc) {
            let mut our_sc = ScopeContext::new_unrooted(Scopes::all(), self.key.clone());
            self.cache
                .borrow_mut()
                .insert(key.loc.clone(), our_sc.clone());
            Self::validate_bv(&self.bv, data, &mut our_sc);
            sc.expect_compatibility(&our_sc, key);
            self.cache.borrow_mut().insert(key.loc.clone(), our_sc);
        }
    }
}
