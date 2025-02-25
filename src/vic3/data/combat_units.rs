use crate::block::validator::Validator;
use crate::block::Block;
use crate::context::ScopeContext;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::item::Item;
use crate::scopes::Scopes;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;

#[derive(Clone, Debug)]
pub struct CombatUnit {}

impl CombatUnit {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::CombatUnit, key, block, Box::new(Self {}));
    }
}

impl DbKind for CombatUnit {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);

        vd.field_integer("max_manpower");
        vd.field_choice("type", &["army", "navy"]);

        vd.field_item("icon", Item::File);

        vd.field_validated_key_blocks("combat_unit_image", |key, block, data| {
            let mut vd = Validator::new(block, data);
            let mut sc = ScopeContext::new(Scopes::CombatUnit, key);
            vd.field_validated_block("trigger", |block, data| {
                validate_trigger(block, data, &mut sc, Tooltipped::No);
            });
            vd.field_item("texture", Item::File);
        });
    }
}
