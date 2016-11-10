use bit_set::BitSet;
use color::PuyoColor;

const MAX_SIZE: usize = 8;

pub struct ColumnPuyoList {
    size: [usize; 6],
    puyo: [[PuyoColor; 6]; MAX_SIZE],
    place_holders: BitSet,
}

impl ColumnPuyoList {
    pub fn new() -> ColumnPuyoList {
        ColumnPuyoList {
            size: [0; 6],
            puyo: [[PuyoColor::EMPTY; 6]; MAX_SIZE],
            place_holders: BitSet::new(),
        }
    }
}
