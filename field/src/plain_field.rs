use color::{Color, PuyoColor, RealColor};
use field;
use field_checker::FieldChecker;
use position::Position;
use score;

pub struct PlainField<C: Color<C>> {
    field: [[C; field::MAP_HEIGHT]; field::MAP_WIDTH],
}

impl<C: Color<C>> PlainField<C> {
    pub fn new() -> PlainField<C> {
        let w = C::wall_color();
        let e = C::empty_color();
        PlainField {
            field: [
                //        4           8          12          16
                [w, w, w, w, w, w, w, w, w, w, w, w, w, w, w, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, e, e, e, e, e, e, e, e, e, e, e, e, e, e, w],
                [w, w, w, w, w, w, w, w, w, w, w, w, w, w, w, w],
            ]
        }
    }

    pub fn from_str(s: &str) -> PlainField<C> {
        let mut field = Self::new();

        assert!(s.len() % 6 == 0);

        let mut cnt = 0;
        for b in s.bytes().rev() {
            let x = 6 - (cnt % 6);
            let y = (cnt / 6) + 1;
            field.set_color(x, y, C::from_byte(b));
            cnt += 1;
        }

        field
    }

    pub fn color(&self, x: usize, y: usize) -> C {
        // self.field[x as usize][y as usize]

        debug_assert!(x < 8);
        debug_assert!(y < 16);
        unsafe {
            *self.field.get_unchecked(x).get_unchecked(y)
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: C) {
        // self.field[x as usize][y as usize] = color

        debug_assert!(x < 8);
        debug_assert!(y < 16);
        unsafe {
            *self.field.get_unchecked_mut(x).get_unchecked_mut(y) = color
        }
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.color(x, y) == C::empty_color()
    }

    pub fn is_color(&self, x: usize, y: usize, color: C) -> bool {
        self.color(x, y) == color
    }

    pub fn drop(&mut self) {
        for x in 1 .. 7 {
            let mut h = 1;
            for y in 1 .. 14 {
                if self.is_empty(x, y) {
                    continue;
                }
                let c = self.color(x, y);
                self.set_color(x, h, c);
                h += 1;
            }

            while h <= 13 {
                self.set_color(x, h, C::empty_color());
                h += 1;
            }
        }
    }

    // Returns new head.
    pub fn fill_same_color_position(&self, x: usize, y: usize, c: C,
                                    head: usize, queue: &mut [Position; 72],
                                    checker: &mut FieldChecker) -> usize {
        if y > field::HEIGHT {
            return head;
        }

        let mut write_head = head;
        let mut read_head = head;

        debug_assert!(!checker.get(x, y));
        queue[write_head] = Position::new(x, y);
        write_head += 1;
        checker.set(x, y);

        while write_head != read_head {
            let p = queue[read_head];
            read_head += 1;

            if self.is_color(p.x + 1, p.y, c) && !checker.get(p.x + 1, p.y) {
                queue[write_head] = Position::new(p.x + 1, p.y);
                write_head += 1;
                checker.set(p.x + 1, p.y);
            }
            if self.is_color(p.x - 1, p.y, c) && !checker.get(p.x - 1, p.y) {
                queue[write_head] = Position::new(p.x - 1, p.y);
                write_head += 1;
                checker.set(p.x - 1, p.y);
            }
            if self.is_color(p.x, p.y + 1, c) && !checker.get(p.x, p.y + 1) && p.y + 1 <= field::HEIGHT {
                queue[write_head] = Position::new(p.x, p.y + 1);
                write_head += 1;
                checker.set(p.x, p.y + 1);
            }
            if self.is_color(p.x, p.y - 1, c) && !checker.get(p.x, p.y - 1) {
                queue[write_head] = Position::new(p.x, p.y - 1);
                write_head += 1;
                checker.set(p.x, p.y - 1);
            }
        }

        return write_head;
    }

    pub fn count_connected_puyos(&self, x: usize, y: usize) -> usize {
        let mut checker = FieldChecker::new();
        self.count_connected_puyos_with_checker(x, y, &mut checker)
    }

    pub fn count_connected_puyos_with_checker(&self, x: usize, y: usize, checker: &mut FieldChecker) -> usize {
        let mut positions = [Position::new(0, 0); 72];
        self.fill_same_color_position(x, y, self.color(x, y), 0, &mut positions, checker)
    }

    pub fn count_connected_puyos_max4(&self, x: usize, y: usize) -> usize {
        if y > field::HEIGHT {
            return 0;
        }

        let mut left_up = false;
        let mut left_down = false;
        let mut right_up = false;
        let mut right_down = false;

        let mut cnt = 1;
        let c = self.color(x, y);

        if self.is_color(x - 1, y, c) {
            if self.is_color(x - 2, y, c) {
                if self.is_color(x - 3, y, c) {
                    return 4;
                }
                if self.is_color(x - 2, y + 1, c) && y + 1 <= field::HEIGHT {
                    return 4;
                }
                if self.is_color(x - 2, y - 1, c) {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x - 1, y + 1, c) && y + 1 <= field::HEIGHT {
                if self.is_color(x - 2, y + 1, c) {
                    return 4;
                }
                if self.is_color(x - 1, y + 2, c) && y + 2 <= field::HEIGHT {
                    return 4;
                }
                cnt += 1;
                left_up = true;
            }
            if self.is_color(x - 1, y - 1, c) {
                if self.is_color(x - 2, y - 1, c) || self.is_color(x - 1, y - 2, c) {
                    return 4;
                }
                cnt += 1;
                left_down = true;
            }
            cnt += 1;
        }

        if self.is_color(x + 1, y, c) {
            if self.is_color(x + 2, y, c) {
                if self.is_color(x + 3, y, c) {
                    return 4;
                }
                if self.is_color(x + 2, y + 1, c) && y + 1 <= field::HEIGHT {
                    return 4;
                }
                if self.is_color(x + 2, y - 1, c) {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x + 1, y + 1, c) && y + 1 <= field::HEIGHT {
                if self.is_color(x + 2, y + 1, c) {
                    return 4;
                }
                if self.is_color(x + 1, y + 2, c) && y + 2 <= field::HEIGHT {
                    return 4;
                }
                cnt += 1;
                right_up = true;
            }
            if self.is_color(x + 1, y - 1, c) {
                if self.is_color(x + 2, y - 1, c) {
                    return 4;
                }
                if self.is_color(x + 1, y - 2, c) {
                    return 4;
                }
                cnt += 1;
                right_down = true;
            }
            cnt += 1;
        }

        if self.is_color(x, y - 1, c) {
            if self.is_color(x, y - 2, c) {
                if self.is_color(x, y - 3, c) {
                    return 4;
                }
                if self.is_color(x - 1, y - 2, c) {
                    return 4;
                }
                if self.is_color(x + 1, y - 2, c) {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x - 1, y - 1, c) && !left_down {
                if self.is_color(x - 2, y - 1, c) {
                    return 4;
                }
                if self.is_color(x - 1, y - 2, c) {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x + 1, y - 1, c) && !right_down {
                if self.is_color(x + 2, y - 1, c) {
                    return 4;
                }
                if self.is_color(x + 1, y - 2, c) {
                    return 4;
                }
                cnt += 1;
            }
            cnt += 1;
        }

        if self.is_color(x, y + 1, c) && y + 1 <= field::HEIGHT {
            if self.is_color(x, y + 2, c) && y + 2 <= field::HEIGHT {
                if self.is_color(x, y + 3, c) && y + 3 <= field::HEIGHT {
                    return 4;
                }
                if self.is_color(x - 1, y + 2, c) {
                    return 4;
                }
                if self.is_color(x + 1, y + 2, c) {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x - 1, y + 1, c) && !left_up {
                if self.is_color(x - 2, y + 1, c) {
                    return 4;
                }
                if self.is_color(x - 1, y + 2, c) && y + 2 <= field::HEIGHT {
                    return 4;
                }
                cnt += 1;
            }
            if self.is_color(x + 1, y + 1, c) && !right_up {
                if self.is_color(x + 2, y + 1, c) {
                    return 4;
                }
                if self.is_color(x + 1, y + 2, c) && y + 2 <= field::HEIGHT {
                    return 4;
                }
                cnt += 1;
            }
            cnt += 1;
        }

        cnt
    }

    pub fn vanish(&mut self, current_chain: i32) -> i32 {
        let mut checker = FieldChecker::new();
        // All the positions of erased puyos will be stored here.
        let mut erase_queue : [Position; 72] = [Position::new(0, 0); 72];
        let mut erase_queue_head = 0;

        let mut used_colors : [bool; 8] = [false; 8];
        let mut num_used_colors = 0;
        let mut long_bonus_coef: i32 = 0;

        for x in 1 .. (field::WIDTH + 1) {
            for y in 1 .. (field::HEIGHT + 1) {
                if self.is_empty(x, y) || checker.get(x, y) || !self.color(x, y).is_normal_color() {
                    continue;
                }

                let c = self.color(x, y);
                let new_head = self.fill_same_color_position(x, y, c, erase_queue_head, &mut erase_queue, &mut checker);

                let connected_puyo_num = new_head - erase_queue_head;
                if connected_puyo_num < 4 {
                    continue;
                }

                erase_queue_head = new_head;
                long_bonus_coef += score::long_bonus(connected_puyo_num as i32);
                if !used_colors[c.as_usize()] {
                    num_used_colors += 1;
                    used_colors[c.as_usize()] = true;
                }
            }
        }

        if erase_queue_head == 0 {
            return 0;
        }

        // --- Actually erase the Puyos to be vanished. We erase ojama here also.
        for i in 0 .. erase_queue_head {
            let x = erase_queue[i].x;
            let y = erase_queue[i].y;

            self.set_color(x, y, C::empty_color());

            // Check OJAMA puyos erased
            if self.is_color(x + 1, y, C::ojama_color()) {
                self.set_color(x + 1, y, C::empty_color());
            }

            if self.is_color(x - 1, y, C::ojama_color()) {
                self.set_color(x - 1, y, C::empty_color());
            }

            // We don't need to update minHeights here.
            if self.is_color(x, y + 1, C::ojama_color()) && y + 1 <= field::HEIGHT {
                self.set_color(x, y + 1, C::empty_color());
            }

            if self.is_color(x, y - 1, C::ojama_color()) {
                self.set_color(x, y - 1, C::empty_color());
            }
        }

        let rensa_bonus_coef: i32 = score::calculate_rensa_bonus_coef(
            score::chain_bonus(current_chain),
            long_bonus_coef,
            score::color_bonus(num_used_colors)
        );
        10 * (erase_queue_head as i32) * rensa_bonus_coef
    }
}

impl<C> PartialEq<PlainField<C>> for PlainField<C>
where C: Color<C> {
    fn eq(&self, other: &PlainField<C>) -> bool {
        // TODO(mayah): Would be good memory comparison.
        for x in 0..8 {
            for y in 0..16 {
                if self.color(x, y) != other.color(x, y) {
                    return false
                }
            }
        }

        true
    }
}

pub type PuyoPlainField = PlainField<PuyoColor>;
pub type RealPlainField = PlainField<RealColor>;

#[cfg(test)]
mod tests {
    use std::mem;
    use color::PuyoColor;
    use field;
    use plain_field::PuyoPlainField;

    #[test]
    fn test_memory_size() {
        assert_eq!(mem::size_of::<PuyoPlainField>(), 128);
    }

    #[test]
    fn test_color() {
        let pf = PuyoPlainField::from_str("RGYB@&");
        assert_eq!(pf.color(0, 1), PuyoColor::WALL);
        assert_eq!(pf.color(1, 1), PuyoColor::RED);
        assert_eq!(pf.color(2, 1), PuyoColor::GREEN);
        assert_eq!(pf.color(3, 1), PuyoColor::YELLOW);
        assert_eq!(pf.color(4, 1), PuyoColor::BLUE);
        assert_eq!(pf.color(5, 1), PuyoColor::OJAMA);
        assert_eq!(pf.color(6, 1), PuyoColor::IRON);
        assert_eq!(pf.color(7, 1), PuyoColor::WALL);

        assert_eq!(pf.color(1, 2), PuyoColor::EMPTY);
    }

    #[test]
    fn test_is_color() {
        let pf = PuyoPlainField::from_str("RGYB@&");
        assert!(pf.is_color(0, 1, PuyoColor::WALL));
        assert!(pf.is_color(1, 1, PuyoColor::RED));
        assert!(pf.is_color(2, 1, PuyoColor::GREEN));
        assert!(pf.is_color(3, 1, PuyoColor::YELLOW));
        assert!(pf.is_color(4, 1, PuyoColor::BLUE));
        assert!(pf.is_color(5, 1, PuyoColor::OJAMA));
        assert!(pf.is_color(6, 1, PuyoColor::IRON));
        assert!(pf.is_color(7, 1, PuyoColor::WALL));

        assert!(pf.is_color(1, 2, PuyoColor::EMPTY));
    }

    #[test]
    fn test_is_empty() {
        let pf = PuyoPlainField::from_str("RGYB@&");
        assert!(pf.is_empty(1, 2));
        assert!(!pf.is_empty(1, 1));
    }

    #[test]
    fn test_drop() {
        let mut pf = PuyoPlainField::from_str(concat!(
            "RRRBBB",
            "......",
            "RRRBBB",
            "......",
            "RRRBBB"));
        let expected = PuyoPlainField::from_str(concat!(
            "RRRBBB",
            "RRRBBB",
            "RRRBBB"));

        pf.drop();
        assert!(pf == expected);
    }

    #[test]
    fn test_drop_extreme() {
        let mut pf = PuyoPlainField::from_str(concat!(
            "OOOOOO", // 14
            "OOOOOO", // 13
            "......", // 12
            "......",
            "......",
            "......",
            "......", // 8
            "......",
            "......",
            "......",
            "......", // 4
            "......",
            "......",
            "......"));

        let expected = PuyoPlainField::from_str(concat!(
            "OOOOOO", // 14
            "......", // 13
            "......", // 12
            "......",
            "......",
            "......",
            "......", // 8
            "......",
            "......",
            "......",
            "......", // 4
            "......",
            "......",
            "OOOOOO"));

        pf.drop();
        assert!(pf == expected);
    }

    #[test]
    fn test_count_connected_puyos() {
        // I O S Z L J T
        let fi = PuyoPlainField::from_str(concat!(
            "R.....",
            "R.....",
            "R.....",
            "R.....",
            "......",
            "RRRR.."));
        let fo = PuyoPlainField::from_str(concat!(
            "RR....",
            "RR...."));
        let fs = PuyoPlainField::from_str(concat!(
            "....R.",
            ".RR.RR",
            "RR...R"));
        let fz = PuyoPlainField::from_str(concat!(
            ".....R",
            "RR..RR",
            ".RR.R."));
        let fl = PuyoPlainField::from_str(concat!(
            "RR....",
            ".R...R",
            ".R.RRR",
            "R.....",
            "R..RRR",
            "RR.R.."));
        let fj = PuyoPlainField::from_str(concat!(
            "RR....",
            "R..R..",
            "R..RRR",
            ".R....",
            ".R.RRR",
            "RR...R"));
        let ft = PuyoPlainField::from_str(concat!(
            ".R....",
            "RR..R.",
            ".R.RRR",
            "R.....",
            "RR.RRR",
            "R...R."));

        let fields = [fi, fo, fs, fz, fl, fj, ft];
        for pf in fields.iter() {
            for x in 1 .. (field::WIDTH + 1) {
                for y in 1 .. (field::HEIGHT + 1) {
                    if !pf.is_color(x, y, PuyoColor::RED) {
                        continue;
                    }

                    assert_eq!(4, pf.count_connected_puyos(x, y));
                    assert_eq!(4, pf.count_connected_puyos_max4(x, y));
                }
            }
        }
    }

    #[test]
    fn test_count_connected_puyos_edge_case() {
        let pf = PuyoPlainField::from_str(concat!(
            "RRRBBB",
            "RRRBBB", // 12
            "......",
            "......",
            "......",
            "......", // 8
            "......",
            "......",
            "......",
            "......", // 4
            "......",
            "......",
            "......",
        ));

        for x in 1 .. (field::WIDTH + 1) {
            assert_eq!(3, pf.count_connected_puyos(x, 12));
            assert_eq!(3, pf.count_connected_puyos_max4(x, 12));
            assert_eq!(0, pf.count_connected_puyos(x, 13));
            assert_eq!(0, pf.count_connected_puyos_max4(x, 13));
        }
    }
}
