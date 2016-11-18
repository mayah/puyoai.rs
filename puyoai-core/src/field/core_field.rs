use color::PuyoColor;
use column_puyo_list::ColumnPuyoList;
use decision::Decision;
use field::{self, BitField, FieldHeight, FieldIsEmpty};
use std;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn height(&self, x: usize) -> usize {
        self.height[x] as usize
    }

    pub fn is_color(&self, x: usize, y: usize, c: PuyoColor) -> bool {
        self.field.is_color(x, y, c)
    }

    pub fn is_normal_color(&self, x: usize, y: usize) -> bool {
        self.field.is_normal_color(x, y)
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.field.is_empty(x, y)
    }

    pub fn field(&self) -> &BitField {
        &self.field
    }

    pub fn is_chigiri_decision(&self, decision: &Decision) -> bool {
        debug_assert!(decision.valid(), "decision {:?} should be valid", decision);
        let axis_x = decision.axis_x();
        let child_x = decision.child_x();
        if axis_x == child_x {
            return false;
        }

        self.height(axis_x) != self.height(child_x)
    }

    pub fn is_connected(&self, x: usize, y: usize) -> bool {
        self.field.is_connected(x, y)
    }

    pub fn count_connected_max4(&self, x: usize, y: usize) -> usize {
        self.field.count_connected_max4(x, y)
    }

    pub fn count_connected_max4_with_color(&self, x: usize, y: usize, c: PuyoColor) -> usize {
        self.field.count_connected_max4_with_color(x, y, c)
    }

    pub fn drop_puyo_on_with_max_height(&mut self, x: usize, c: PuyoColor, max_height: usize) -> bool {
        debug_assert!(c != PuyoColor::EMPTY);
        debug_assert!(max_height <= 14);

        if self.height(x) >= std::cmp::min(13, max_height) {
            return false;
        }

        debug_assert!(self.color(x, self.height(x) + 1) == PuyoColor::EMPTY,
                      "x={} max_height={}", x, max_height);

        self.height[x] += 1;
        self.field.set_color(x, self.height[x] as usize, c);

        true
    }

    pub fn drop_column_puyo_list(&mut self, cpl: &ColumnPuyoList) -> bool {
        self.drop_column_puyo_list_with_max_height(cpl, 13)
    }

    pub fn drop_column_puyo_list_with_max_height(&mut self, cpl: &ColumnPuyoList, max_height: usize) -> bool {
        // check size
        for x in 1..7 {
            if self.height(x) + cpl.size_on(x) > max_height {
                return false;
            }
        }

        for x in 1..7 {
            let s = cpl.size_on(x);
            for i in 0..s {
                self.height[x] += 1;
                let c = cpl.get(x, i);
                let h = self.height(x);
                self.field.set_color(x, h, c);
            }
        }

        true
    }
}

impl FieldHeight for CoreField {
    fn height(&self, x: usize) -> usize {
        CoreField::height(self, x)
    }
}

impl FieldIsEmpty for CoreField {
    fn is_empty(&self, x: usize, y: usize) -> bool {
        CoreField::is_empty(self, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::CoreField;
    use color::PuyoColor;
    use column_puyo_list::ColumnPuyoList;
    use decision::Decision;
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

    #[test]
    fn test_drop_puyo_on() {
        let mut cf = CoreField::from_str(concat!(
            ".....R", // 13
            "OOOOOR", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO"));

        assert!(cf.drop_puyo_on_with_max_height(3, PuyoColor::BLUE, 13));
        assert!(cf.drop_puyo_on_with_max_height(4, PuyoColor::BLUE, 13));
        assert!(!cf.drop_puyo_on_with_max_height(4, PuyoColor::BLUE, 13));
        assert!(!cf.drop_puyo_on_with_max_height(6, PuyoColor::BLUE, 13));

        let expected = CoreField::from_str(concat!(
            "..BB.R", // 13
            "OOOOOR", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO"));

        assert_eq!(cf, expected);
    }

    #[test]
    fn test_drop_column_puyo_list_on() {
        let mut cf = CoreField::from_str(concat!(
            ".....R", // 13
            "OOOOOR", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO"));

        let mut cpl = ColumnPuyoList::new();
        assert!(cpl.add(3, PuyoColor::BLUE));
        assert!(cpl.add(4, PuyoColor::BLUE));
        assert!(cf.drop_column_puyo_list_with_max_height(&cpl, 13));

        let expected = CoreField::from_str(concat!(
            "..BB.R", // 13
            "OOOOOR", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO"));

        assert_eq!(cf, expected);
    }

    #[test]
    fn test_is_chigiri_decision_1() {
        let cf = CoreField::new();

        assert!(!cf.is_chigiri_decision(&Decision::new(3, 0)));
        assert!(!cf.is_chigiri_decision(&Decision::new(3, 1)));
        assert!(!cf.is_chigiri_decision(&Decision::new(3, 2)));
        assert!(!cf.is_chigiri_decision(&Decision::new(3, 3)));
    }

    #[test]
    fn test_is_chigiri_decision_2() {
        let cf = CoreField::from_str("..O...");

        assert!(!cf.is_chigiri_decision(&Decision::new(3, 0)));
        assert!(cf.is_chigiri_decision(&Decision::new(3, 1)));
        assert!(!cf.is_chigiri_decision(&Decision::new(3, 2)));
        assert!(cf.is_chigiri_decision(&Decision::new(3, 3)));
    }
}
