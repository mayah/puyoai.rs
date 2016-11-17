use sseext;
use std;
use x86intrin::*;

#[derive(Clone, Copy, Debug)]
pub struct FieldBit {
    m: m128i,
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

    pub fn mask(&self, mask: FieldBit) -> FieldBit {
        *self & mask
    }

    pub fn not_mask(&self, mask: FieldBit) -> FieldBit {
        FieldBit::new(mm_andnot_si128(mask.m, self.m))
    }

    pub fn andnot(&self, other: FieldBit) -> FieldBit {
        FieldBit::new(mm_andnot_si128(self.m, other.m))
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

    pub fn not_masked_field_13(&self) -> FieldBit {
        let r = sseext::mm_setone_si128() ^ mm_setr_epi16(0, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0);
        FieldBit {
            m: mm_and_si128(self.m, r),
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

    /// Returns bits where edge is expanded.
    /// This might contain the original bits, so you'd like to take mask.
    ///
    /// ```text
    /// ......      ..xx..    ..xx..
    /// ..xx..  --> .x..x. or .xxxx.
    /// ......      ..xx..    ..xx..
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use puyocore::field_bit::FieldBit;
    /// let fb = FieldBit::from_str(concat!(
    ///     "......",
    ///     "..11..",
    ///     "......"));
    /// let expected = FieldBit::from_str(concat!(
    ///     "..11..",
    ///     ".1111.",
    ///     "..11.."));
    /// assert_eq!(expected, fb.expand_edge() | fb);
    /// ```
    pub fn expand_edge(&self) -> FieldBit {
        let seed = self.m;
        let m1 = mm_slli_epi16(seed, 1);
        let m2 = mm_srli_epi16(seed, 1);
        let m3 = mm_slli_si128(seed, 2);
        let m4 = mm_srli_si128(seed, 2);

        return FieldBit::new((m1 | m2) | (m3 | m4))
    }

    pub fn iterate_bit_with_masking<F: FnMut(FieldBit) -> FieldBit>(&self, mut callback: F) {
        let zero = mm_setzero_si128();
        let down_ones = mm_cvtsi64_si128(-1 as i64);
        let up_ones = mm_slli_si128(down_ones, 8);

        let mut current = self.m;

        // upper is zero?
        while mm_testz_si128(up_ones, current) == 0 {
            // y = x & (-x)
            let y = mm_and_si128(current, mm_sub_epi64(zero, current));
            let z = mm_and_si128(up_ones, y);
            let mask = callback(FieldBit::new(z));
            current = mm_andnot_si128(mask.as_m128i(), current);
        }

        while mm_testz_si128(down_ones, current) == 0 {
            // y = x & (-x)
            let y = mm_and_si128(current, mm_sub_epi64(zero, current));
            let z = mm_and_si128(down_ones, y);
            let mask = callback(FieldBit::new(z));
            current = mm_andnot_si128(mask.as_m128i(), current);
        }
    }

    pub fn iterate_bit_position<F>(&self, mut callback: F) where F: FnMut(usize, usize) {
        let mut low = self.m.as_u64x2().extract(0);
        let mut high = self.m.as_u64x2().extract(1);

        while low != 0 {
            let bit = low.trailing_zeros();
            let x = bit >> 4;
            let y = bit & 0xF;
            callback(x as usize, y as usize);
            low = low & (low - 1);
        }

        while high != 0 {
            let bit = high.trailing_zeros();
            let x = 4 + (bit >> 4);
            let y = bit & 0xF;
            callback(x as usize, y as usize);
            high = high & (high - 1);
        }
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

    #[test]
    fn test_iterate_bit_position() {
        let bf = FieldBit::from_str(concat!(
            "...1..", // 6
            ".1....", // 5
            "......", // 4
            "..1...", // 3
            "...1..", // 2
            "1.....", // 1
        ));

        let mut s = Vec::new();
        bf.iterate_bit_position(|x, y| {
            s.push((x, y));
        });
        s.sort();

        assert_eq!(&[(1, 1), (2, 5), (3, 3), (4, 2), (4, 6)], s.as_slice());
    }
}
