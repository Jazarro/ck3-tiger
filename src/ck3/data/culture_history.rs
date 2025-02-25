use crate::block::validator::Validator;
use crate::block::Block;
use crate::date::Date;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::item::Item;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct CultureHistory {}

impl CultureHistory {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::CultureHistory, key, block, Box::new(Self {}));
    }
}

impl DbKind for CultureHistory {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        if key.starts_with("heritage_") {
            data.verify_exists(Item::CultureHeritage, key);
        } else {
            data.verify_exists(Item::Culture, key);
        }

        let mut vd = Validator::new(block, data);
        vd.validate_history_blocks(validate_history);
    }
}

fn validate_history(_date: Date, block: &Block, data: &Everything) {
    let mut vd = Validator::new(block, data);

    vd.field_items("discover_innovation", Item::Innovation);
    vd.field_validated_blocks("add_innovation_progress", |block, data| {
        let mut vd = Validator::new(block, data);
        vd.field_item("culture_innovation", Item::Innovation);
        vd.field_numeric_range("progress", 0.0, 100.0);
    });
    vd.field_item("join_era", Item::CultureEra);
    vd.field_numeric_range("progress_era", 0.0, 100.0);
}
