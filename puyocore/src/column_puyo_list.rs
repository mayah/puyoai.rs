use color::PuyoColor;
use small_int_set::SmallIntSet;

const MAX_SIZE: usize = 8;

pub struct ColumnPuyoList {
    size: [usize; 6],
    puyo: [[PuyoColor; 6]; MAX_SIZE],
    place_holders: SmallIntSet,
}

impl ColumnPuyoList {
    pub fn new() -> ColumnPuyoList {
        ColumnPuyoList {
            size: [0; 6],
            puyo: [[PuyoColor::EMPTY; 6]; MAX_SIZE],
            place_holders: SmallIntSet::new(),
        }
    }
}
