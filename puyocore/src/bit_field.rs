use color::{self, PuyoColor};
use field;
use field_bit::FieldBit;
use plain_field::PuyoPlainField;
use sseext;
use std::{self, mem};
use x86intrin::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BitField {
    m: [FieldBit; 3],
}

impl BitField {
    pub fn new() -> BitField {
        BitField {
            m: [FieldBit::empty(),
                FieldBit::from_values(0xFFFF, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0xFFFF),
                FieldBit::empty(), ]
        }
    }

    pub fn uninitialized() -> BitField {
        BitField {
            m: [FieldBit::uninitialized(), FieldBit::uninitialized(), FieldBit::uninitialized()]
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

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        let whole = self.m[0] | self.m[1] | self.m[2];
        return !(whole.get(x, y))
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

    pub fn count_connected_puyos(&self, x: usize, y: usize) -> usize {
        if y > field::HEIGHT {
            return 0
        }

        let c = self.color(x, y);
        let color_bits = self.bits(c).masked_field_12();
        FieldBit::from_onebit(x, y).expand(&color_bits).popcount()
    }

    pub fn bits(&self, c: PuyoColor) -> FieldBit {
        let r0 = self.m[0].as_m128i();
        let r1 = self.m[1].as_m128i();
        let r2 = self.m[2].as_m128i();

        let v = match c {
            PuyoColor::EMPTY => {  // 0
                let x = mm_or_si128(mm_or_si128(r0, r1), r2);
                mm_xor_si128(x, mm_setr_epi32(!0, !0, !0, !0))
            },
            PuyoColor::OJAMA => {  // 1
                mm_andnot_si128(r2, mm_andnot_si128(r1, r0))
            },
            PuyoColor::WALL => {   // 2
                mm_andnot_si128(r2, mm_andnot_si128(r0, r1))
            },
            PuyoColor::IRON => {   // 3
                mm_andnot_si128(r2, mm_and_si128(r0, r1))
            },
            PuyoColor::RED => {    // 4
                mm_andnot_si128(r0, mm_andnot_si128(r1, r2))
            },
            PuyoColor::BLUE => {   // 5
                mm_and_si128(r0, mm_andnot_si128(r1, r2))
            },
            PuyoColor::YELLOW => { // 6
                mm_andnot_si128(r0, mm_and_si128(r1, r2))
            },
            PuyoColor::GREEN => {  // 7
                mm_and_si128(r0, mm_and_si128(r1, r2))
            },
        };

        FieldBit::new(v)
    }

    pub fn escape_invisible(&mut self) -> BitField {
        let mut escaped = BitField::uninitialized();
        for i in 0 .. 3 {
            escaped.m[i] = self.m[i].not_masked_field_13();
            self.m[i] = self.m[i].masked_field_13();
        }

        escaped
    }

    pub fn recover_invisible(&mut self, bf: &BitField) {
        for i in 0 .. 3 {
            self.m[i].set_all(bf.m[i]);
        }
    }

    pub fn vanish_fast(&self, erased: &mut FieldBit) -> bool {
        *erased = FieldBit::empty();
        let mut did_erase = false;

        for c in &color::NORMAL_PUYO_COLORS {
            let mask = self.bits(*c).masked_field_12();
            let mut vanishing = FieldBit::uninitialized();
            if !mask.find_vanishing_bits(&mut vanishing) {
                continue
            }

            erased.set_all(vanishing);
            did_erase = true
        }

        if !did_erase {
            return false;
        }

        let ojama_erased = erased.expand1(self.bits(PuyoColor::OJAMA)).masked_field_12();
        erased.set_all(ojama_erased);

        // tracker->trackVanish(currentChain, *erased, ojamaErased);
        return true;
    }

    pub fn drop_after_vanish_fast(&mut self, erased: FieldBit) {
        let ones = sseext::mm_setone_si128();

        let t = mm_xor_si128(erased.as_m128i(), ones);
        let old_low_bits = t.as_u64x2().extract(0);
        let old_high_bits = t.as_u64x2().extract(1);

        let shift = mm256_cvtepu16_epi32(sseext::mm_popcnt_epi16(erased.as_m128i()));
        let half_ones = mm256_cvtepu16_epi32(ones);
        let mut shifted = mm256_srlv_epi32(half_ones, shift);
        shifted = mm256_packus_epi32(shifted, shifted);

        let new_low_bits = shifted.as_u64x4().extract(0);
        let new_high_bits = shifted.as_u64x4().extract(2);

        let mut d = [self.m[0].as_m128i().as_u64x2(), self.m[1].as_m128i().as_u64x2(), self.m[2].as_m128i().as_u64x2()];

        if new_low_bits != 0xFFFFFFFFFFFFFFFF {
            d[0] = d[0].insert(0, pdep_u64(pext_u64(d[0].extract(0), old_low_bits), new_low_bits));
            d[1] = d[1].insert(0, pdep_u64(pext_u64(d[1].extract(0), old_low_bits), new_low_bits));
            d[2] = d[2].insert(0, pdep_u64(pext_u64(d[2].extract(0), old_low_bits), new_low_bits));
            if new_high_bits != 0xFFFFFFFFFFFFFFFF {
                d[0] = d[0].insert(1, pdep_u64(pext_u64(d[0].extract(1), old_high_bits), new_high_bits));
                d[1] = d[1].insert(1, pdep_u64(pext_u64(d[1].extract(1), old_high_bits), new_high_bits));
                d[2] = d[2].insert(1, pdep_u64(pext_u64(d[2].extract(1), old_high_bits), new_high_bits));
            }
        } else {
            d[0] = d[0].insert(1, pdep_u64(pext_u64(d[0].extract(1), old_high_bits), new_high_bits));
            d[1] = d[1].insert(1, pdep_u64(pext_u64(d[1].extract(1), old_high_bits), new_high_bits));
            d[2] = d[2].insert(1, pdep_u64(pext_u64(d[2].extract(1), old_high_bits), new_high_bits));
        }

        self.m[0] = FieldBit::new(d[0].as_m128i());
        self.m[1] = FieldBit::new(d[1].as_m128i());
        self.m[2] = FieldBit::new(d[2].as_m128i());
    }
}

impl std::fmt::Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO(mayah): More sophisticated way?

        for y in 0 .. 16 {
            for x in 0 .. 8 {
                write!(f, "{}", self.color(x, 15 - y));
            }
            writeln!(f, "{}", "");
        }

        write!(f, "{}", "")
    }
}

#[cfg(test)]
mod tests {
    use super::BitField;
    use color;
    use color::PuyoColor;
    use field;
    use field_bit::FieldBit;

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
    fn test_is_empty() {
        let bf = BitField::from_str(concat!(
            "RRR..."));

        assert!(!bf.is_empty(1, 1));
        assert!(!bf.is_empty(2, 1));
        assert!(!bf.is_empty(3, 1));
        assert!(bf.is_empty(4, 1));
        assert!(bf.is_empty(5, 1));
        assert!(bf.is_empty(6, 1));
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
                for c in color::ALL_PUYO_COLORS.iter() {
                    assert_eq!(bf.bits(*c).get(x, y), *c == bf.color(x, y));
                    assert_eq!(bf.is_color(x, y, *c), bf.is_color(x, y, *c));
                }

                assert_eq!(bf.is_normal_color(x, y), bf.is_normal_color(x, y));
            }
        }
    }

    #[test]
    fn test_count_connected_puyos() {
        let bf = BitField::from_str(concat!(
            "RRRRRR",
            "BYBRRY",
            "RRRBBB"));

        assert_eq!(bf.count_connected_puyos(1, 1), 3);
        assert_eq!(bf.count_connected_puyos(4, 1), 3);
        assert_eq!(bf.count_connected_puyos(1, 2), 1);
        assert_eq!(bf.count_connected_puyos(3, 2), 1);
        assert_eq!(bf.count_connected_puyos(6, 2), 1);
        assert_eq!(bf.count_connected_puyos(4, 2), 8);
    }

    #[test]
    fn test_vanish_fast_1() {
        let bf = BitField::from_str(concat!(
            "..YY..",
            "GGGGYY",
            "RRRROY"));

        let expected = FieldBit::from_str(concat!(
            "1111..",
            "11111."));

        let mut vanishing = FieldBit::uninitialized();
        assert!(bf.vanish_fast(&mut vanishing));
        assert_eq!(expected, vanishing);
    }

    #[test]
    fn test_vanish_fast_2() {
        let bf = BitField::from_str(concat!(
            "OOOOOO",
            "OOGGOO",
            "OOGGOO"));

        let expected = FieldBit::from_str(concat!(
            "..11..",
            ".1111.",
            ".1111."));

        let mut vanishing = FieldBit::uninitialized();
        assert!(bf.vanish_fast(&mut vanishing));
        assert_eq!(expected, vanishing);
    }

    #[test]
    fn test_vanish_fast_3() {
        let bf = BitField::from_str(concat!(
            "....RR", // 13
            "OO.ORR", // 12
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

        let mut vanishing = FieldBit::uninitialized();
        assert!(!bf.vanish_fast(&mut vanishing));
    }

    #[test]
    fn test_drop_after_vanish_fast() {
        let mut bf = BitField::from_str(concat!(
            "..BB..",
            "RRRR.."));
        let erased = FieldBit::from_str(concat!(
            "1111.."));

        let invisible = bf.escape_invisible();
        bf.drop_after_vanish_fast(erased);
        bf.recover_invisible(&invisible);

        let expected = BitField::from_str(concat!(
            "..BB.."));

        assert_eq!(expected, bf);
    }
}
