use color::{Color, PuyoColor, RealColor};
use field;

pub struct PlainField<C: Color<C> + Copy + PartialEq<C>> {
    field: [[C; field::MAP_HEIGHT]; field::MAP_WIDTH],
}

impl<C: Color<C> + Copy + PartialEq<C>> PlainField<C> {
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

    pub fn color(&self, x: i32, y: i32) -> C {
        // self.field[x as usize][y as usize]

        debug_assert!(0 <= x && x < 8);
        debug_assert!(0 <= y && y < 16);
        unsafe {
            *self.field.get_unchecked(x as usize).get_unchecked(y as usize)
        }
    }

    pub fn set_color(&mut self, x: i32, y: i32, color: C) {
        // self.field[x as usize][y as usize] = color

        debug_assert!(0 <= x && x < 8);
        debug_assert!(0 <= y && y < 16);
        unsafe {
            *self.field.get_unchecked_mut(x as usize).get_unchecked_mut(y as usize) = color
        }
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.color(x, y) == C::empty_color()
    }

    pub fn is_color(&self, x: i32, y: i32, color: C) -> bool {
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
}

impl<C> PartialEq<PlainField<C>> for PlainField<C>
where C: Color<C> + Copy + PartialEq<C> {
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
}
