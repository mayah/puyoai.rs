use field_bit::FieldBit;
use std;
use x86intrin::*;

#[derive(Clone, Copy, Debug)]
pub struct FieldBit256 {
    m: m256i,
}

impl FieldBit256 {
    pub fn new(m: m256i) -> FieldBit256 {
        FieldBit256 {
            m: m,
        }
    }

    pub unsafe fn uninitialized() -> FieldBit256 {
        std::mem::uninitialized::<FieldBit256>()
    }

    pub fn empty() -> FieldBit256 {
        FieldBit256 {
            m: mm256_setzero_si256()
        }
    }

    pub fn from_low_high(low: FieldBit, high: FieldBit) -> FieldBit256 {
        let m = mm256_inserti128_si256(mm256_castsi128_si256(low.as_m128i()), high.as_m128i(), 1);
        FieldBit256 {
            m: m
        }
    }

    #[allow(dead_code)]
    pub fn get_low(&self, x: usize, y: usize) -> bool {
        self.low().get(x, y)
    }

    #[allow(dead_code)]
    pub fn get_high(&self, x: usize, y: usize) -> bool {
        self.high().get(x, y)
    }

    pub fn low(&self) -> FieldBit {
        FieldBit::new(mm256_castsi256_si128(self.m))
    }

    pub fn high(&self) -> FieldBit {
        FieldBit::new(mm256_extracti128_si256(self.m, 1))
    }

    #[allow(dead_code)]
    pub fn set_low(&mut self, x: usize, y: usize) {
        self.m = self.m | FieldBit256::onebit_low(x, y)
    }

    #[allow(dead_code)]
    pub fn set_high(&mut self, x: usize, y: usize) {
        self.m = self.m | FieldBit256::onebit_high(x, y)
    }

    pub fn set_all(&mut self, fb: FieldBit256) {
        self.m = self.m | fb.m
    }

    #[allow(dead_code)]
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

    pub fn find_vanishing_bits(&self, vanishing: &mut FieldBit256) -> bool {
        let m = self.m;
        let u = mm256_srli_epi16(m, 1) & m;
        let d = mm256_slli_epi16(m, 1) & m;
        let l = mm256_slli_si256(m, 2) & m;
        let r = mm256_srli_si256(m, 2) & m;

        let ud_and = u & d;
        let lr_and = l & r;
        let ud_or = u | d;
        let lr_or = l | r;

        let twos = lr_and | ud_and | (ud_or & lr_or);
        let two_d = mm256_slli_epi16(twos, 1) & twos;
        let two_l = mm256_slli_si256(twos, 2) & twos;
        let threes = (ud_and & lr_or) | (lr_and & ud_or);
        let t = two_d | two_l | threes;

        if mm256_testz_si256(t, t) != 0 {
            *vanishing = FieldBit256::empty();
            return false;
        }

        let two_u = mm256_srli_epi16(twos, 1) & twos;
        let two_r = mm256_srli_si256(twos, 2) & twos;
        *vanishing = FieldBit256::new(t | two_u | two_r).expand1(*self);
        return true;
    }

    pub fn popcount_low_high(&self) -> (usize, usize) {
        (self.low().popcount(), self.high().popcount())
    }

    fn check_in_range(x: usize, y: usize) -> bool {
        x < 8 && y < 16
    }

    fn onebit_low(x: usize, y: usize) -> m256i {
        debug_assert!(FieldBit256::check_in_range(x, y));

        let shift = ((x << 4) | y) & 0x3F;
        let zero = mm256_setzero_si256();
        if x < 4 {
            return mm256_insert_epi64(zero, 1 << shift, 0)
        } else {
            return mm256_insert_epi64(zero, 1 << shift, 1)
        }
    }

    fn onebit_high(x: usize, y: usize) -> m256i {
        debug_assert!(FieldBit256::check_in_range(x, y));

        let shift = ((x << 4) | y) & 0x3F;
        let zero = mm256_setzero_si256();
        if x < 4 {
            return mm256_insert_epi64(zero, 1 << shift, 2)
        } else {
            return mm256_insert_epi64(zero, 1 << shift, 3)
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
    use super::*;
    use field;
    use field_bit::FieldBit;

    #[test]
    fn test_constructor() {
        let fb256 = FieldBit256::empty();

        for x in 0 .. 8 {
            for y in 0 .. 16 {
                assert!(!fb256.get_low(x, y));
                assert!(!fb256.get_high(x, y));
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
        assert!(fb256.get_low(1, 3));
        assert!(fb256.get_low(4, 8));
        assert!(fb256.get_high(2, 4));
        assert!(fb256.get_high(5, 9));

        assert!(!fb256.get_high(1, 3));
        assert!(!fb256.get_high(4, 8));
        assert!(!fb256.get_low(2, 4));
        assert!(!fb256.get_low(5, 9));

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

    #[test]
    fn test_find_vanishing_bits_1() {
        let fb256 = {
            let f = FieldBit::from_str(concat!(
                ".1....",
                "11..1.",
                ".1.111",
                "1...1.",
                "11.111",
                "1...1."));
            FieldBit256::from_low_high(f, f)
        };

        let mut vanishing = unsafe { FieldBit256::uninitialized() };
        assert!(fb256.find_vanishing_bits(&mut vanishing));

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. field::HEIGHT + 1 {
                assert_eq!(vanishing.get_low(x, y), fb256.get_low(x, y), "x={}, y={}", x, y);
                assert_eq!(vanishing.get_high(x, y), fb256.get_high(x, y), "x={}, y={}", x, y);
            }
        }
    }

    #[test]
    fn test_find_vanishing_bits_2() {
        let fb256 = {
            let f = FieldBit::from_str(concat!(
                ".....1",
                ".111.1",
                ".....1",
                ".1.11."));
            FieldBit256::from_low_high(f, f)
        };

        let mut vanishing = unsafe { FieldBit256::uninitialized() };
        assert!(!fb256.find_vanishing_bits(&mut vanishing));

        for x in 1 .. field::WIDTH + 1 {
            for y in 1 .. field::HEIGHT + 1 {
                assert!(!vanishing.get_low(x, y));
                assert!(!vanishing.get_high(x, y));
            }
        }
    }

    #[test]
    fn test_iterate_bit_with_masking() {
        let mut bf = FieldBit::empty();
        bf.set(1, 2);
        bf.set(2, 3);
        bf.set(3, 4);
        bf.set(4, 5);
        bf.set(5, 6);
        bf.set(6, 7);

        assert_eq!(6, bf.popcount());

        let mut count = 0;
        let mut iterated = FieldBit::empty();
        bf.iterate_bit_with_masking(|x: FieldBit| -> FieldBit {
            iterated.set_all(x);
            assert_eq!(1, x.popcount());
            count += 1;

            x
        });

        assert_eq!(6, count);
        assert_eq!(bf, iterated);
    }
}
