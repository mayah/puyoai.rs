use std;
use x86intrin::*;

#[derive(Clone, Copy, Debug)]
pub struct FieldBit {
    m: m128i,
}

#[derive(Clone, Copy, Debug)]
pub struct FieldBit256 {
    m: m256i,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LowHigh {
    LOW,
    HIGH,
}

impl FieldBit {
    pub fn new(m: m128i) -> FieldBit {
        FieldBit {
            m: m,
        }
    }

    pub fn uninitialized() -> FieldBit {
        unsafe { std::mem::uninitialized::<FieldBit>() }
    }

    pub fn empty() -> FieldBit {
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
        let mut f = FieldBit::empty();

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
        mm_testz_si128(FieldBit::onebit(x, y), self.m) == 0
    }

    pub fn set(&mut self, x: usize, y: usize) {
        debug_assert!(FieldBit::check_in_range(x, y));
        self.m = mm_or_si128(FieldBit::onebit(x, y), self.m)
    }

    pub fn set_all(&mut self, fb: FieldBit) {
        self.m = mm_or_si128(self.m, fb.m)
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        debug_assert!(FieldBit::check_in_range(x, y));
        self.m = mm_andnot_si128(FieldBit::onebit(x, y), self.m)
    }

    pub fn is_empty(&self) -> bool {
        mm_testz_si128(self.m, self.m) != 0
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

    /// Returns true if there are 4-connected bits.
    /// Such bits are copied to `vanishing`.
    pub fn find_vanishing_bits(&self, vanishing: &mut FieldBit) -> bool {
        //  x
        // xox              -- o is 3-connected
        //
        // xoox  ox   x oo
        //      xo  xoo oo  -- o is 2-connected.
        //
        // So, one 3-connected piece or two 2-connected pieces are necessary and sufficient.
        //
        // Also, 1-connected won't be connected to each other in vanishing case.
        // So, after this, expand1() should be enough.

        let u = mm_srli_epi16(self.m, 1) & self.m;
        let d = mm_slli_epi16(self.m, 1) & self.m;
        let l = mm_slli_si128(self.m, 2) & self.m;
        let r = mm_srli_si128(self.m, 2) & self.m;

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let threes = (ud_and & lr_or) | (lr_and & ud_or);
        let twos = ud_and | lr_and | (ud_or & lr_or);

        let two_d = mm_slli_epi16(twos, 1) & twos;
        let two_l = mm_slli_si128(twos, 2) & twos;

        let mut t = threes | two_d | two_l;
        if mm_testz_si128(t, t) != 0 {
            *vanishing = FieldBit::empty();
            return false;
        }

        let two_u = mm_srli_epi16(twos, 1) & twos;
        let two_r = mm_srli_si128(twos, 2) & twos;
        t = t | two_u | two_r;

        *vanishing = FieldBit::new(t).expand1(*self);
        return true;
    }

    pub fn has_vanishing_bits(&self) -> bool {
        let u = mm_and_si128(mm_srli_epi16(self.m, 1), self.m);
        let d = mm_and_si128(mm_slli_epi16(self.m, 1), self.m);
        let l = mm_and_si128(mm_slli_si128(self.m, 2), self.m);
        let r = mm_and_si128(mm_srli_si128(self.m, 2), self.m);

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let threes = (ud_and & lr_or) | (lr_and & ud_or);
        let twos = ud_and | lr_and | (ud_or & lr_or);

        let two_d = mm_slli_epi16(twos, 1) & twos;
        let two_l = mm_slli_si128(twos, 2) & twos;

        let vanishing = threes | two_d | two_l;
        return mm_testz_si128(vanishing, vanishing) == 0;
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
                return FieldBit::new(expanded);
            }
            seed = expanded;
        }
    }

    pub fn expand1(&self, mask: FieldBit) -> FieldBit {
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

impl std::ops::BitOr for FieldBit {
    type Output = FieldBit;

    fn bitor(self, rhs: FieldBit) -> FieldBit {
        FieldBit::new(mm_or_si128(self.m, rhs.m))
    }
}

impl std::ops::BitAnd for FieldBit {
    type Output = FieldBit;

    fn bitand(self, rhs: FieldBit) -> FieldBit {
        FieldBit::new(mm_and_si128(self.m, rhs.m))
    }
}

impl std::cmp::PartialEq<FieldBit> for FieldBit {
    fn eq(&self, other: &FieldBit) -> bool {
        let x = mm_xor_si128(self.m, other.m);
        mm_testz_si128(x, x) == 1
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

impl FieldBit256 {
    pub fn new(m: m256i) -> FieldBit256 {
        FieldBit256 {
            m: m,
        }
    }

    pub fn uninitialized() -> FieldBit256 {
        unsafe { std::mem::uninitialized::<FieldBit256>() }
    }

    pub fn empty() -> FieldBit256 {
        FieldBit256 {
            m: mm256_setzero_si256()
        }
    }

    pub fn from_low_high(low: FieldBit, high: FieldBit) -> FieldBit256 {
        let m = mm256_inserti128_si256(mm256_castsi128_si256(low.m), high.m, 1);
        FieldBit256 {
            m: m
        }
    }

    pub fn get(&self, lowhigh: LowHigh, x: usize, y: usize) -> bool {
        debug_assert!(FieldBit256::check_in_range(x, y));
        mm256_testz_si256(FieldBit256::onebit(lowhigh, x, y), self.m) == 0
    }

    pub fn low(&self) -> FieldBit {
        FieldBit::new(mm256_castsi256_si128(self.m))
    }

    pub fn high(&self) -> FieldBit {
        FieldBit::new(mm256_extracti128_si256(self.m, 1))
    }

    pub fn set_low(&mut self, x: usize, y: usize) {
        self.m = self.m | FieldBit256::onebit(LowHigh::LOW, x, y)
    }

    pub fn set_high(&mut self, x: usize, y: usize) {
        self.m = self.m | FieldBit256::onebit(LowHigh::HIGH, x, y)
    }

    pub fn expand(&self, mask: FieldBit256) -> FieldBit256 {
        let mut seed = self.m;

        loop {
            let mut expanded = seed;
            expanded = mm256_slli_epi16(seed, 1) | expanded;
            expanded = mm256_srli_epi16(seed, 1) | expanded;
            expanded = mm256_slli_si256(seed, 2) | expanded;
            expanded = mm256_srli_si256(seed, 2) | expanded;
            expanded = mask.m & expanded;

            if mm256_testc_si256(seed, expanded) != 0 { // seed == expanded
                return FieldBit256::new(expanded);
            }
            seed = expanded;
        }
    }

    pub fn expand1(&self, mask: FieldBit256) -> FieldBit256 {
        let v1 = mm256_slli_si256(self.m, 2);
        let v2 = mm256_srli_si256(self.m, 2);
        let v3 = mm256_slli_epi16(self.m, 1);
        let v4 = mm256_srli_epi16(self.m, 1);
        FieldBit256::new(((self.m | v1) | (v2 | v3) | v4) & mask.m)
    }

    fn check_in_range(x: usize, y: usize) -> bool {
        x < 8 && y < 16
    }

    fn onebit(lowhigh: LowHigh, x: usize, y: usize) -> m256i {
        debug_assert!(FieldBit256::check_in_range(x, y));

        // TODO(mayah): Maybe we have more good solution.

        let shift = ((x << 4) | y) & 0x3F;
        let zero = mm256_setzero_si256();
        if lowhigh == LowHigh::LOW {
            if x < 4 {
                return mm256_insert_epi64(zero, 1 << shift, 0)
            } else {
                return mm256_insert_epi64(zero, 1 << shift, 1)
            }
        } else {
            if x < 4 {
                return mm256_insert_epi64(zero, 1 << shift, 2)
            } else {
                return mm256_insert_epi64(zero, 1 << shift, 3)
            }
        }
    }
}

impl std::ops::BitOr for FieldBit256 {
    type Output = FieldBit256;

    fn bitor(self, rhs: FieldBit256) -> FieldBit256 {
        FieldBit256::new(self.m | rhs.m)
    }
}

impl std::ops::BitAnd for FieldBit256 {
    type Output = FieldBit256;

    fn bitand(self, rhs: FieldBit256) -> FieldBit256 {
        FieldBit256::new(self.m & rhs.m)
    }
}

#[cfg(test)]
mod tests {
    use field;
    use field_bit::FieldBit;

    #[test]
    fn test_empty() {
        let fb = FieldBit::empty();
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
                let mut fb = FieldBit::empty();
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

        let actual = seed.expand1(mask);
        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert_eq!(actual.get(x, y), expected.get(x, y), "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_is_empty() {
        let fb1 = FieldBit::empty();
        assert!(fb1.is_empty());

        let fb2 = FieldBit::from_onebit(1, 3);
        assert!(!fb2.is_empty());
    }

    #[test]
    fn test_set_all() {
        let mut fb = FieldBit::empty();
        let fb1 = FieldBit::from_onebit(1, 3);
        let fb2 = FieldBit::from_onebit(2, 4);
        fb.set_all(fb1);
        fb.set_all(fb2);

        assert!(fb.get(1, 3));
        assert!(fb.get(2, 4));
        assert!(!fb.get(1, 4));
        assert!(!fb.get(2, 3));
    }

    #[test]
    fn test_find_vanishing_bits_1() {
        let f = FieldBit::from_str(concat!(
            ".1....",
            "11..1.",
            ".1.111",
            "1...1.",
            "11.111",
            "1...1."));

        let mut vanishing = FieldBit::uninitialized();
        assert!(f.has_vanishing_bits());
        assert!(f.find_vanishing_bits(&mut vanishing));

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. field::HEIGHT + 1 {
                assert_eq!(vanishing.get(x, y), f.get(x, y), "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_find_vanishing_bits_2() {
        let f = FieldBit::from_str(concat!(
            ".....1",
            ".111.1",
            ".....1",
            ".1.11."));

        let mut vanishing = FieldBit::uninitialized();
        assert!(!f.has_vanishing_bits());
        assert!(!f.find_vanishing_bits(&mut vanishing));

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. field::HEIGHT + 1 {
                assert!(!vanishing.get(x, y));
            }
        }
    }

    #[test]
    fn test_eq() {
        let fb1 = FieldBit::from_str(concat!(
            "1....1",
            "1....1",
            "111111"));
        let fb2 = FieldBit::from_str(concat!(
            "111111",
            "1....1",
            "1....1"));

        assert!(fb1 == fb1);
        assert!(fb2 == fb2);
        assert!(fb1 != fb2);
    }

    #[test]
    fn test_bit() {
        let fb1 = FieldBit::from_str(concat!(
            "1....1",
            "1....1",
            "111111"));
        let fb2 = FieldBit::from_str(concat!(
            "111111",
            "1....1",
            "1....1"));

        let expected_and = FieldBit::from_str(concat!(
            "1....1",
            "1....1",
            "1....1"));
        let expected_or = FieldBit::from_str(concat!(
            "111111",
            "1....1",
            "111111"));

        assert_eq!(expected_and, fb1 & fb2);
        assert_eq!(expected_or, fb1 | fb2);
    }
}

#[cfg(test)]
mod field_bit_256_tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let fb256 = FieldBit256::empty();

        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert!(!fb256.get(LowHigh::LOW, x, y));
                assert!(!fb256.get(LowHigh::HIGH, x, y));
            }
        }
    }

    #[test]
    fn test_from_low_high() {
        let mut low = FieldBit::empty();
        let mut high = FieldBit::empty();
        low.set(1, 3);
        low.set(4, 8);
        high.set(2, 4);
        high.set(5, 9);

        let fb256 = FieldBit256::from_low_high(low, high);
        assert!(fb256.get(LowHigh::LOW, 1, 3));
        assert!(fb256.get(LowHigh::LOW, 4, 8));
        assert!(fb256.get(LowHigh::HIGH, 2, 4));
        assert!(fb256.get(LowHigh::HIGH, 5, 9));

        assert!(!fb256.get(LowHigh::HIGH, 1, 3));
        assert!(!fb256.get(LowHigh::HIGH, 4, 8));
        assert!(!fb256.get(LowHigh::LOW, 2, 4));
        assert!(!fb256.get(LowHigh::LOW, 5, 9));

        assert_eq!(low, fb256.low());
        assert_eq!(high, fb256.high());
    }

    #[test]
    fn test_expand() {
        let mask_high = FieldBit::from_str(concat!(
            "......",
            "11..11",
            "11..11",
            "......",
            "111111"));
        let mask_low = FieldBit::from_str(concat!(
            "111111",
            ".....1",
            "111111",
            "1.....",
            "111111"));

        let expected_high = FieldBit::from_str(concat!(
            "111111"));
        let expected_low = mask_low;

        let mask = FieldBit256::from_low_high(mask_low, mask_high);

        let mut bit = FieldBit256::empty();
        bit.set_high(3, 1);
        bit.set_low(6, 1);

        let expanded = bit.expand(mask);
        assert_eq!(expected_high, expanded.high());
        assert_eq!(expected_low, expanded.low());
    }
}
