use color::PuyoColor;
use field::{self, BitField};

pub struct CoreField {
    field: BitField,
    height: [i16; 8],
}

impl CoreField {
    pub fn new() -> CoreField {
        CoreField {
            field: BitField::new(),
            height: [0; 8],
        }
    }

    pub fn from_str(s: &str) -> CoreField {
        let mut cf = CoreField {
            field: BitField::from_str(s),
            height: [0; 8],
        };

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. 15 {
                if cf.is_empty(x, y) {
                    cf.height[x] = (y - 1) as i16;
                    break
                }
            }
        }

        cf
    }

    pub fn color(&self, x: usize, y: usize) -> PuyoColor {
        self.field.color(x, y)
    }

    pub fn is_color(&self, x: usize, y: usize, c: PuyoColor) -> bool {
        self.field.is_color(x, y, c)
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.field.is_empty(x, y)
    }

    pub fn height(&self, x: usize) -> i16 {
        self.height[x]
    }
}

#[cfg(test)]
mod tests {
    use super::CoreField;

    use color::PuyoColor;
    use field;

    #[test]
    fn test_constructor() {
        let cf = CoreField::new();

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. field::HEIGHT + 1 {
                assert!(cf.is_empty(x, y));
                assert!(cf.is_color(x, y, PuyoColor::EMPTY));
                assert_eq!(PuyoColor::EMPTY, cf.color(x, y));
            }
        }

        for x in 1 .. field::WIDTH + 1 {
            assert_eq!(0, cf.height(x));
        }
    }

    #[test]
    fn test_from_str() {
        let cf = CoreField::from_str(concat!(
            "R.....",
            "RRRB.."));

        assert_eq!(PuyoColor::WALL, cf.color(0, 1));
        assert_eq!(PuyoColor::RED, cf.color(1, 1));
        assert_eq!(PuyoColor::RED, cf.color(2, 1));
        assert_eq!(PuyoColor::RED, cf.color(3, 1));
        assert_eq!(PuyoColor::BLUE, cf.color(4, 1));
        assert_eq!(PuyoColor::EMPTY, cf.color(5, 1));
        assert_eq!(PuyoColor::EMPTY, cf.color(6, 1));
        assert_eq!(PuyoColor::WALL, cf.color(7, 1));

        assert_eq!(2, cf.height(1));
        assert_eq!(1, cf.height(2));
        assert_eq!(1, cf.height(3));
        assert_eq!(1, cf.height(4));
        assert_eq!(0, cf.height(5));
        assert_eq!(0, cf.height(6));
    }
}
