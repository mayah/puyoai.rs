use color::PuyoColor;
use field;
use field_bit::FieldBit;
use plain_field::PuyoPlainField;
use simd;
use simdext::*;
use std::mem;

#[derive(Clone, Copy)]
pub struct BitField {
    m: [FieldBit; 3],
}

impl BitField {
    pub fn new() -> BitField {
        BitField {
            m: [FieldBit::new_empty(),
                FieldBit::from_values(0xFFFF, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0xFFFF),
                FieldBit::new_empty(), ]
        }
    }

    pub fn from_plain_field(pf: PuyoPlainField) -> BitField {
        let mut bf = BitField::new();

        // TODO(mayah): We have better algorithm here.
        for x in 0 .. field::MAP_WIDTH {
            for y in 0 .. field::MAP_HEIGHT {
                bf.set_color(x, y, pf.color(x, y))
            }
        }

        bf
    }

    pub fn from_str(s: &str) -> BitField {
        BitField::from_plain_field(PuyoPlainField::from_str(s))
    }

    pub fn color(&self, x: usize, y: usize) -> PuyoColor {
        let b0: u8 = if self.m[0].get(x, y) { 1 } else { 0 };
        let b1: u8 = if self.m[1].get(x, y) { 2 } else { 0 };
        let b2: u8 = if self.m[2].get(x, y) { 4 } else { 0 };

        unsafe {
            mem::transmute(b0 | b1 | b2)
        }
    }

    pub fn is_color(&self, x: usize, y: usize, c: PuyoColor) -> bool {
        self.bits(c).get(x, y)
    }

    pub fn is_normal_color(&self, x: usize, y: usize) -> bool {
        self.m[2].get(x, y)
    }

    pub fn set_color(&mut self, x: usize, y: usize, c: PuyoColor) {
        let cc = c as u8;
        for i in 0 .. 3 {
            if (cc & (1 << i)) != 0 {
                self.m[i as usize].set(x, y);
            } else {
                self.m[i as usize].unset(x, y);
            }
        }
    }

    pub fn bits(&self, c: PuyoColor) -> FieldBit {
        let v = match c {
            PuyoColor::EMPTY => {  // 0
                (self.r(0) | self.r(1) | self.r(2)) ^ simd::u16x8::splat(0xFFFF)
            },
            PuyoColor::OJAMA => {  // 1
                mm_andnot_si128(self.r(2), mm_andnot_si128(self.r(1), self.r(0)))
            },
            PuyoColor::WALL => {   // 2
                mm_andnot_si128(self.r(2), mm_andnot_si128(self.r(0), self.r(1)))
            },
            PuyoColor::IRON => {   // 3
                mm_andnot_si128(self.r(2), mm_and_si128(self.r(0), self.r(1)))
            },
            PuyoColor::RED => {    // 4
                mm_andnot_si128(self.r(0), mm_andnot_si128(self.r(1), self.r(2)))
            },
            PuyoColor::BLUE => {   // 5
                mm_and_si128(self.r(0), mm_andnot_si128(self.r(1), self.r(2)))
            },
            PuyoColor::YELLOW => { // 6
                mm_andnot_si128(self.r(0), mm_and_si128(self.r(1), self.r(2)))
            },
            PuyoColor::GREEN => {  // 7
                mm_and_si128(self.r(0), mm_and_si128(self.r(1), self.r(2)))
            },
        };

        FieldBit::new(v)
    }

    fn r(&self, i: usize) -> simd::u16x8 {
        self.m[i].as_u16x8()
    }
}

#[cfg(test)]
mod tests {
    use super::BitField;
    use color;
    use color::PuyoColor;
    use field;

    #[test]
    fn test_initial() {
        let bf = BitField::new();
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                if x == 0 || x == 7 || y == 0 || y == 15 {
                    assert_eq!(bf.color(x, y), PuyoColor::WALL);
                } else {
                    assert_eq!(bf.color(x, y), PuyoColor::EMPTY);
                }
            }
        }
    }

    #[test]
    fn test_from_str() {
        let bf = BitField::from_str(concat!(
            "RGBRGB",
            "RGBRGB",
            "RGBRGB"));

        assert_eq!(bf.color(1, 1), PuyoColor::RED);
        assert_eq!(bf.color(2, 1), PuyoColor::GREEN);
        assert_eq!(bf.color(3, 1), PuyoColor::BLUE);
        assert_eq!(bf.color(1, 3), PuyoColor::RED);
        assert_eq!(bf.color(2, 3), PuyoColor::GREEN);
        assert_eq!(bf.color(3, 3), PuyoColor::BLUE);
    }

    #[test]
    fn test_set_color() {
        let colors = [
            PuyoColor::EMPTY, PuyoColor::OJAMA, PuyoColor::WALL, PuyoColor::IRON,
            PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN,
        ];
        let mut bf = BitField::new();

        for c in colors.iter() {
            bf.set_color(1, 1, *c);
            assert_eq!(*c, bf.color(1, 1));
        }
    }

    #[test]
    fn test_each_cell() {
        let bf = BitField::from_str(concat!(
            "&&&&&&",
            "OOOOOO",
            "YYYYYY",
            "BBBBBB",
            "GGGGGG",
            "RRRRRR"));

        for x in 0 .. field::MAP_WIDTH {
            for y in 0 .. field::MAP_HEIGHT {
                for c in color::color::ALL_PUYO_COLORS.iter() {
                    assert_eq!(bf.bits(*c).get(x, y), *c == bf.color(x, y));
                    assert_eq!(bf.is_color(x, y, *c), bf.is_color(x, y, *c));
                }

                assert_eq!(bf.is_normal_color(x, y), bf.is_normal_color(x, y));
            }
        }
    }
}
