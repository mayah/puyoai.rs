use color::PuyoColor;
use column_puyo_list::ColumnPuyoList;
use decision::Decision;
use field::{self, BitField, FieldHeight, FieldIsEmpty};
use frame;

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

    /// Returns true if ZENKESHI.
    pub fn is_all_cleared(&self) -> bool {
        self.field.is_all_cleared()
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

    pub fn frames_to_drop_next(&self, decision: &Decision) -> usize {
        // TODO(mayah): This calculation should be more accurate. We need to compare this with
        // actual AC puyo2 and duel server algorithm. These must be much the same.

        // TODO(mayah): When "kabegoe" happens, we need more frames.
        const KABEGOE_PENALTY: usize = 6;

        let x1 = decision.axis_x();
        let x2 = decision.child_x();

        let mut drop_frames = frame::FRAMES_TO_MOVE_HORIZONTALLY[(3 - x1 as isize).abs() as usize];

        if decision.rot() == 0 {
            let drop_height = field::HEIGHT as isize - self.height(x1) as isize;
            if drop_height <= 0 {
                // TODO(mayah): We need to add penalty here. How much penalty is necessary?
                drop_frames += KABEGOE_PENALTY + frame::FRAMES_GROUNDING;
            } else {
                drop_frames += frame::FRAMES_TO_DROP_FAST[drop_height as usize] + frame::FRAMES_GROUNDING;
            }
        } else if decision.rot() == 2 {
            let mut drop_height = (field::HEIGHT as isize) - (self.height(x1) as isize) - 1;
            // TODO: If puyo lines are high enough, rotation might take time. We should measure this later.
            // It looks we need 3 frames to waiting that each rotation has completed.
            if drop_height < 6 {
                drop_height = 6;
            }

            drop_frames += frame::FRAMES_TO_DROP_FAST[drop_height as usize] + frame::FRAMES_GROUNDING;
        } else {
            if self.height(x1) == self.height(x2) {
                let drop_height = field::HEIGHT as isize - self.height(x1) as isize;
                if drop_height <= 0 {
                    drop_frames += KABEGOE_PENALTY + frame::FRAMES_GROUNDING;
                } else if drop_height < 3 {
                    drop_frames += frame::FRAMES_TO_DROP_FAST[3] + frame::FRAMES_GROUNDING;
                } else {
                    drop_frames += frame::FRAMES_TO_DROP_FAST[drop_height as usize] + frame::FRAMES_GROUNDING;
                }
            } else {
                let min_height = std::cmp::min(self.height(x1), self.height(x2));
                let max_height = std::cmp::max(self.height(x1), self.height(x2));
                let diff_height = max_height - min_height;
                let drop_height = field::HEIGHT as isize - max_height as isize;
                if drop_height <= 0 {
                    drop_frames += KABEGOE_PENALTY;
                } else if drop_height < 3 {
                    drop_frames += frame::FRAMES_TO_DROP_FAST[3];
                } else {
                    drop_frames += frame::FRAMES_TO_DROP_FAST[drop_height as usize];
                }
                drop_frames += frame::FRAMES_GROUNDING;
                drop_frames += frame::FRAMES_TO_DROP[diff_height as usize];
                drop_frames += frame::FRAMES_GROUNDING;
            }
        }

        drop_frames
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
    use frame;

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

    #[test]
    fn test_frames_to_drop_next_without_chigiri() {
        // TODO(mayah): We have to confirm this.
        let cf = CoreField::new();

        assert_eq!(frame::FRAMES_TO_DROP_FAST[field::HEIGHT] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(3, 0)));
        assert_eq!(frame::FRAMES_TO_DROP_FAST[field::HEIGHT] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(3, 1)));
        assert_eq!(frame::FRAMES_TO_DROP_FAST[field::HEIGHT - 1] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(3, 2)));
        assert_eq!(frame::FRAMES_TO_DROP_FAST[field::HEIGHT] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(3, 3)));
        assert_eq!(frame::FRAMES_TO_MOVE_HORIZONTALLY[2] + frame::FRAMES_TO_DROP_FAST[field::HEIGHT] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(1, 0)));
    }

    #[test]
    fn test_frames_to_drop_next_with_chigiri() {
        let cf = CoreField::from_str(concat!(
            "..O...",
            "..O...",
            "..O...",
            "..O...",
        ));

        let expected = frame::FRAMES_TO_DROP_FAST[field::HEIGHT - 4] + frame::FRAMES_GROUNDING +
            frame::FRAMES_TO_DROP[4] + frame::FRAMES_GROUNDING;
        assert_eq!(expected, cf.frames_to_drop_next(&Decision::new(3, 1)));
    }

    #[test]
    fn test_frames_to_drop_next_on_13th_row() {
        let cf = CoreField::from_str(concat!(
            "OO.OOO", // 12
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
            "OOOOOO",
        ));

        assert_eq!(11, cf.height(3));
        assert_eq!(12, cf.height(4));

        // We cannot put with Decision(4, 2).

        assert_eq!(6 + frame::FRAMES_TO_MOVE_HORIZONTALLY[1] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(4, 0)));
        assert_eq!(6 + frame::FRAMES_TO_MOVE_HORIZONTALLY[1] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(4, 1)));
        assert_eq!(6 + frame::FRAMES_TO_MOVE_HORIZONTALLY[1] + frame::FRAMES_GROUNDING + frame::FRAMES_TO_DROP[1] + frame::FRAMES_GROUNDING,
                   cf.frames_to_drop_next(&Decision::new(4, 3)));
    }
}
