use std;
use x86intrin::*;

#[inline]
unsafe fn bitcast<T, U>(x: T) -> U {
    debug_assert!(std::mem::size_of::<T>() == std::mem::size_of::<U>());
    std::mem::transmute_copy(&x)
}

#[derive(Clone, Copy)]
pub struct FieldBit {
    m: m128i,
}

impl FieldBit {
    pub fn new(m: m128i) -> FieldBit {
        FieldBit {
            m: m,
        }
    }

    pub fn new_empty() -> FieldBit {
        FieldBit {
            m: mm_setzero_si128()
        }
    }

    pub fn from_values(v1: u16, v2: u16, v3: u16, v4: u16, v5: u16, v6: u16, v7: u16, v8: u16) -> FieldBit {
        FieldBit {
            m: mm_setr_epi16(v1 as i16, v2 as i16, v3 as i16, v4 as i16,
                             v5 as i16, v6 as i16, v7 as i16, v8 as i16)
        }
    }

    pub fn from_onebit(x: usize, y: usize) -> FieldBit {
        FieldBit {
            m: FieldBit::onebit(x, y)
        }
    }

    pub fn from_str(s: &str) -> FieldBit {
        let mut f = FieldBit::new_empty();

        assert!(s.len() % 6 == 0);

        let mut cnt = 0;
        for b in s.bytes().rev() {
            let x = 6 - (cnt % 6);
            let y = (cnt / 6) + 1;
            if b == b'1' {
                f.set(x, y);
            }
            cnt += 1;
        }

        f
    }

    pub fn as_m128i(&self) -> m128i {
        self.m
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        debug_assert!(FieldBit::check_in_range(x, y));
        unsafe {
            mm_testz_si128(FieldBit::onebit(x, y), self.m) == 0
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        debug_assert!(FieldBit::check_in_range(x, y));
        self.m = mm_or_si128(FieldBit::onebit(x, y), self.m)
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        debug_assert!(FieldBit::check_in_range(x, y));
        self.m = mm_andnot_si128(FieldBit::onebit(x, y), self.m)
    }

    pub fn masked_field_12(&self) -> FieldBit {
        FieldBit {
            m: mm_and_si128(self.m, mm_setr_epi16(0, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0)),
        }
    }

    pub fn masked_field_13(&self) -> FieldBit {
        FieldBit {
            m: mm_and_si128(self.m, mm_setr_epi16(0, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0)),
        }
    }

    pub fn popcount(&self) -> usize {
        let x = self.m.as_u64x2();
        let low: u64 = x.extract(0);
        let high: u64 = x.extract(1);
        (low.count_ones() + high.count_ones()) as usize
    }

    pub fn expand(&self, mask: &FieldBit) -> FieldBit {
        let mut seed = self.m;
        loop {
            let mut expanded = seed;
            expanded = mm_or_si128(mm_slli_epi16(seed, 1), expanded);
            expanded = mm_or_si128(mm_srli_epi16(seed, 1), expanded);
            expanded = mm_or_si128(mm_slli_si128(seed, 2), expanded);
            expanded = mm_or_si128(mm_srli_si128(seed, 2), expanded);
            expanded = mm_and_si128(mask.m, expanded);

            if mm_testc_si128(seed, expanded) != 0 { // seed == expanded
                return FieldBit { m: expanded };
            }
            seed = expanded;
        }
    }

    pub fn expand1(&self, mask: &FieldBit) -> FieldBit {
        let seed = self.m;
        let v1 = mm_slli_epi16(seed, 1);
        let v2 = mm_srli_epi16(seed, 1);
        let v3 = mm_slli_si128(seed, 2);
        let v4 = mm_srli_si128(seed, 2);

        let m = ((seed | v1) | (v2 | v3) | v4) & mask.m;
        FieldBit { m: m }
    }

    fn check_in_range(x: usize, y: usize) -> bool {
        x < 8 && y < 16
    }

    fn onebit(x: usize, y: usize) -> m128i {
        debug_assert!(FieldBit::check_in_range(x, y));

        let shift = ((x << 4) | y) & 0x3F;
        let hi: i64 = (x as i64) >> 2;
        let lo: i64 = hi ^ 1;

        mm_set_epi64x(hi << shift, lo << shift)
    }
}

impl std::fmt::Display for FieldBit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x = self.m.as_u16x8();
        write!(f, "({}, {}, {}, {}, {}, {}, {}, {})",
               x.extract(0), x.extract(1), x.extract(2), x.extract(3),
               x.extract(4), x.extract(5), x.extract(6), x.extract(7))
    }
}

#[cfg(test)]
mod tests {
    use field;
    use field_bit::FieldBit;

    #[test]
    fn test_empty() {
        let fb = FieldBit::new_empty();
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert_eq!(fb.get(x, y), false);
            }
        }
    }

    #[test]
    fn test_from_value() {
        let fb = FieldBit::from_values(1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7);
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert_eq!(fb.get(x, y), x == y, "failed at x={}, y={}, fb.get(x, y)={}", x, y, fb.get(x, y));
            }
        }
    }

    #[test]
    fn test_from_str() {
        let fb = FieldBit::from_str(concat!(
            "111...",
            "......",
            "111..."));

        for x in 0 .. 8 {
            for y in 0 .. 16 {
                let b = (y == 1 || y == 3) && (1 <= x && x <= 3);
                assert_eq!(fb.get(x, y), b, "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_set_get() {
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                let mut fb = FieldBit::new_empty();
                assert!(!fb.get(x, y));
                fb.set(x, y);
                assert!(fb.get(x, y));
                fb.unset(x, y);
                assert!(!fb.get(x, y));
            }
        }
    }

    #[test]
    fn test_masked_field() {
        let fb = FieldBit::from_values(0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF);
        let fb12 = fb.masked_field_12();
        let fb13 = fb.masked_field_13();

        for x in 0 .. field::MAP_WIDTH {
            for y in 0 .. field::MAP_HEIGHT {
                assert!(fb.get(x, y), "x={}, y={}", x, y);
                assert_eq!(fb12.get(x, y), 1 <= x && x <= 6 && 1 <= y && y <= 12, "x={}, y={}", x, y);
                assert_eq!(fb13.get(x, y), 1 <= x && x <= 6 && 1 <= y && y <= 13, "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_popcount() {
        let fb = FieldBit::from_values(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(fb.popcount(), 1 + 1 + 2 + 1 + 2 + 2 + 3 + 1)
    }

    #[test]
    fn test_expand() {
        let mask = FieldBit::from_str(concat!(
            ".1....",
            "1.11..",
            "1.1...",
            "1.1...",
        ));

        let expected = FieldBit::from_str(concat!(
            "..11..",
            "..1...",
            "..1..."));

        let actual = FieldBit::from_onebit(3, 1).expand(&mask);
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert_eq!(actual.get(x, y), expected.get(x, y), "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_expand1() {
        let mask = FieldBit::from_str(concat!(
            "111111",
            "111111",
            "111111"));

        let seed = FieldBit::from_str(concat!(
            "......",
            "1...1.",
            "......"));

        let expected = FieldBit::from_str(concat!(
            "1...1.",
            "11.111",
            "1...1."));

        let actual = seed.expand1(&mask);
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert_eq!(actual.get(x, y), expected.get(x, y), "x={}, y={}", x, y);
            }
        }
    }
}
