use crate::block::validator::Validator;
use crate::block::Block;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::item::Item;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct Dynasty {}

impl Dynasty {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Dynasty, key, block, Box::new(Self {}));
    }
}

impl DbKind for Dynasty {
    fn validate(&self, _key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        vd.req_field("name");
        vd.field_item("name", Item::Localization);
        vd.field_item("prefix", Item::Localization);
        vd.field_item("motto", Item::Localization);
        vd.field_item("culture", Item::Culture);
        vd.field_value("forced_coa_religiongroup"); // TODO: figure out the values
    }
}
