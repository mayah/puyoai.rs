use color::{Color, PuyoColor};
use field::{self, FieldIsEmpty, PuyoPlainField};
use field_bit::FieldBit;
use field_bit_256::FieldBit256;
use frame;
use rensa_result::RensaResult;
use rensa_tracker::{RensaTracker, RensaNonTracker};
use score;
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

    /// Returns FieldBit where normal color bit is set.
    pub fn normal_color_bits(&self) -> FieldBit {
        self.m[2]
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

    pub fn is_connected(&self, x: usize, y: usize) -> bool {
        self.is_connected_with_color(x, y, self.color(x, y))
    }

    pub fn is_connected_with_color(&self, x: usize, y: usize, c: PuyoColor) -> bool {
        if y > field::HEIGHT {
            return false;
        }

        let color_bits = self.bits(c).masked_field_12();
        let single = FieldBit::from_onebit(x, y);
        !single.expand_edge().mask(color_bits).not_mask(single).is_empty()
    }

    pub fn simulate(&mut self) -> RensaResult {
        let mut tracker = RensaNonTracker::new();
        self.simulate_with_tracker(&mut tracker)
    }

    pub fn simulate_fast(&mut self) -> usize {
        let mut tracker = RensaNonTracker::new();
        self.simulate_fast_with_tracker(&mut tracker)
    }

    pub fn simulate_with_tracker<T: RensaTracker>(&mut self, tracker: &mut T) -> RensaResult {
        let escaped = self.escape_invisible();

        let mut score = 0;
        let mut frames = 0;
        let mut quick = false;
        let mut current_chain = 1;

        loop {
            let mut erased = FieldBit::uninitialized();
            let nth_chain_score = self.vanish(current_chain, &mut erased, tracker);
            if nth_chain_score == 0 {
                break;
            }

            current_chain += 1;
            score += nth_chain_score;
            frames += frame::FRAMES_VANISH_ANIMATION;

            let max_drops = self.drop_after_vanish(erased, tracker);
            if max_drops > 0 {
                frames += frame::FRAMES_TO_DROP_FAST[max_drops] + frame::FRAMES_GROUNDING;
            } else {
                quick = true;
            }
        }

        self.recover_invisible(&escaped);
        RensaResult::new(current_chain - 1, score, frames, quick)
    }

    pub fn simulate_fast_with_tracker<T: RensaTracker>(&mut self, tracker: &mut T) -> usize {
        let escaped = self.escape_invisible();
        let mut current_chain = 1;

        let mut erased = FieldBit::uninitialized();
        while self.vanish_fast(current_chain, &mut erased, tracker) {
            current_chain += 1;
            self.drop_after_vanish_fast(erased, tracker);
        }

        self.recover_invisible(&escaped);
        current_chain - 1
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

    pub fn vanish_fast<T: RensaTracker>(&self, current_chain: usize, erased: &mut FieldBit, tracker: &mut T) -> bool {
        let mut erased256 = FieldBit256::empty();
        let mut did_erase = false;

        // RED (100) & BLUE (101)
        {
            let t = self.m[1].andnot(self.m[2]).masked_field_12();
            let mask = FieldBit256::from_low_high(self.m[0].andnot(t), self.m[0] & t);

            let mut vanishing = FieldBit256::uninitialized();
            if mask.find_vanishing_bits(&mut vanishing) {
                erased256.set_all(vanishing);
                did_erase = true;
            }
        }

        // YELLOW (110) & GREEN (111)
        {
            let t = (self.m[1] & self.m[2]).masked_field_12();
            let mask = FieldBit256::from_low_high(self.m[0].andnot(t), self.m[0] & t);

            let mut vanishing = FieldBit256::uninitialized();
            if mask.find_vanishing_bits(&mut vanishing) {
                erased256.set_all(vanishing);
                did_erase = true;
            }
        }

        if !did_erase {
            *erased = FieldBit::empty();
            return false;
        }

        *erased = erased256.low() | erased256.high();

        let ojama_erased = erased.expand1(self.bits(PuyoColor::OJAMA)).masked_field_12();
        erased.set_all(ojama_erased);

        tracker.track_vanish(current_chain, erased, &ojama_erased);
        true
    }

    pub fn vanish<T: RensaTracker>(&self, current_chain: usize, erased: &mut FieldBit, tracker: &mut T) -> usize {
        let mut erased256 = FieldBit256::empty();

        let mut num_erased_puyos = 0;
        let mut num_colors = 0;
        let mut long_bonus_coef = 0;
        let mut did_erase = false;

        for i in 0 .. 2 {
            let t = (if i == 0 {
                self.m[1].andnot(self.m[2])
            } else {
                self.m[1] & self.m[2]
            }).masked_field_12();

            let high_mask = self.m[0] & t;
            let low_mask = self.m[0].andnot(t);

            let mask = FieldBit256::from_low_high(low_mask, high_mask);
            let mut vanishing = FieldBit256::uninitialized();
            if !mask.find_vanishing_bits(&mut vanishing) {
                continue;
            }
            erased256.set_all(vanishing);
            did_erase = true;

            let (low_count, high_count) = vanishing.popcount_low_high();

            if high_count > 0 {
                num_colors += 1;
                num_erased_puyos += high_count;
                if high_count <= 7 {
                    long_bonus_coef += score::long_bonus(high_count);
                } else {
                    let high = vanishing.high();
                    // slowpath
                    high.iterate_bit_with_masking(|x: FieldBit| -> FieldBit {
                        let expanded = x.expand(&high_mask);
                        long_bonus_coef += score::long_bonus(expanded.popcount());
                        expanded
                    });
                }
            }

            if low_count > 0 {
                num_colors += 1;
                num_erased_puyos += low_count;
                if low_count <= 7 {
                    long_bonus_coef += score::long_bonus(low_count);
                } else {
                    let low = vanishing.low();
                    // slowpath
                    low.iterate_bit_with_masking(|x: FieldBit| -> FieldBit {
                        let expanded = x.expand(&low_mask);
                        long_bonus_coef += score::long_bonus(expanded.popcount());
                        expanded
                    });
                }
            }
        }

        if !did_erase {
            *erased = FieldBit::empty();
            return 0;
        }

        *erased = erased256.low() | erased256.high();

        let color_bonus_coef = score::color_bonus(num_colors);
        let chain_bonus_coef = score::chain_bonus(current_chain);
        let rensa_bonus_coef = score::calculate_rensa_bonus_coef(chain_bonus_coef, long_bonus_coef, color_bonus_coef);

        tracker.track_coef(current_chain, num_erased_puyos, long_bonus_coef, color_bonus_coef);

        // Removes ojama.
        let ojama_erased = erased.expand1(self.bits(PuyoColor::OJAMA)).masked_field_12();
        erased.set_all(ojama_erased);

        tracker.track_vanish(current_chain, erased, &ojama_erased);

        10 * num_erased_puyos * rensa_bonus_coef
    }

    pub fn drop_after_vanish<T: RensaTracker>(&mut self, erased: FieldBit, tracker: &mut T) -> usize {
        // Set 1 at non-empty position.
        // Remove 1 bits from the positions where they are erased.
        let nonempty = mm_andnot_si128(erased.as_m128i(), (self.m[0] | self.m[1] | self.m[2]).as_m128i());

        // Find the holes. The number of holes for each column is the number of
        // drops of the column.
        let holes = mm_and_si128(sseext::mm_porr_epi16(nonempty), erased.as_m128i());
        let num_holes = sseext::mm_popcnt_epi16(holes);
        let max_drops = sseext::mm_hmax_epu16(num_holes);

        self.drop_after_vanish_fast(erased, tracker);

        max_drops as usize
    }

    pub fn drop_after_vanish_fast<T: RensaTracker>(&mut self, erased: FieldBit, tracker: &mut T) {
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

        tracker.track_drop(old_low_bits, old_high_bits, new_low_bits, new_high_bits);
    }
}

impl FieldIsEmpty for BitField {
    fn is_empty(&self, x: usize, y: usize) -> bool {
        BitField::is_empty(self, x, y)
    }
}

impl std::fmt::Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO(mayah): More sophisticated way?
        let mut s = String::new();
        for y in 0 .. 16 {
            for x in 0 .. 8 {
                s.push(self.color(x, 15 - y).to_char());
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::BitField;
    use color::{self, Color, PuyoColor};
    use field;
    use field_bit::FieldBit;
    use frame;
    use rensa_tracker::RensaNonTracker;

    struct SimulationTestcase {
        field: BitField,
        chain: usize,
        score: usize,
        frame: usize,
        quick: bool,
    }

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
                for c in color::PuyoColor::all_colors() {
                    assert_eq!(bf.bits(*c).get(x, y), *c == bf.color(x, y));
                    assert_eq!(bf.is_color(x, y, *c), bf.is_color(x, y, *c));
                }

                assert_eq!(bf.is_normal_color(x, y), bf.is_normal_color(x, y));
            }
        }
    }

    #[test]
    fn test_normal_color_bits() {
        let bf = BitField::from_str("RGO&BY");
        let fb = FieldBit::from_str("11..11");
        assert_eq!(bf.normal_color_bits(), fb);
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
    fn test_is_connected() {
        let bf = BitField::from_str(concat!(
            "B.B..Y",
            "RRRBBB",
        ));

        assert!(bf.is_connected(1, 1));
        assert!(bf.is_connected(2, 1));
        assert!(bf.is_connected(3, 1));
        assert!(bf.is_connected(4, 1));
        assert!(bf.is_connected(5, 1));
        assert!(bf.is_connected(6, 1));
        assert!(!bf.is_connected(1, 2));
        assert!(!bf.is_connected(3, 2));
        assert!(!bf.is_connected(6, 2));
    }

    #[test]
    fn test_is_connected_edge_case() {
        let bf = BitField::from_str(concat!(
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

        assert!(!bf.is_connected(6, 12));
    }

    #[test]
    fn test_simulate() {
        let simulation_testcases = &[
            SimulationTestcase {
                field: BitField::from_str(concat!(
                    ".BBBB.")),
                chain: 1,
                score: 40,
                frame: frame::FRAMES_VANISH_ANIMATION,
                quick: true,
            },
            SimulationTestcase {
                field: BitField::from_str(concat!(
                    ".RBRB.",
                    "RBRBR.",
                    "RBRBR.",
                    "RBRBRR")),
                chain: 5,
                score: 40 + 40 * 8 + 40 * 16 + 40 * 32 + 40 * 64,
                frame: frame::FRAMES_VANISH_ANIMATION * 5 + frame::FRAMES_TO_DROP_FAST[3] * 4 + frame::FRAMES_GROUNDING * 4,
                quick: true,
            },
            SimulationTestcase {
                field: BitField::from_str(concat!(
                    ".YGGY.",
                    "BBBBBB",
                    "GYBBYG",
                    "BBBBBB")),
                chain: 1,
                score: 140 * 10,
                frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[3] + frame::FRAMES_GROUNDING,
                quick: false
            },
        ];

        for testcase in simulation_testcases {
            let mut bf = testcase.field.clone();
            let chain = bf.simulate_fast();
            assert_eq!(testcase.chain, chain)
        }

        for testcase in simulation_testcases {
            let mut bf = testcase.field.clone();
            let rensa_result = bf.simulate();
            assert_eq!(testcase.chain, rensa_result.chain);
            assert_eq!(testcase.score, rensa_result.score);
            assert_eq!(testcase.frame, rensa_result.frame);
            assert_eq!(testcase.quick, rensa_result.quick);
        }
    }

    #[test]
    fn test_vanish_1() {
        let bf = BitField::from_str(concat!(
            "..YY..",
            "GGGGYY",
            "RRRROY"));

        let expected = FieldBit::from_str(concat!(
            "1111..",
            "11111."));

        let mut vanishing = FieldBit::uninitialized();
        let mut tracker = RensaNonTracker::new();
        assert!(bf.vanish_fast(1, &mut vanishing, &mut tracker));
        assert_eq!(expected, vanishing);

        assert_eq!(bf.vanish(1, &mut vanishing, &mut tracker), 80 * 3);
        assert_eq!(expected, vanishing);
    }

    #[test]
    fn test_vanish_2() {
        let bf = BitField::from_str(concat!(
            "OOOOOO",
            "OOGGOO",
            "OOGGOO"));

        let expected = FieldBit::from_str(concat!(
            "..11..",
            ".1111.",
            ".1111."));

        let mut vanishing = FieldBit::uninitialized();
        let mut tracker = RensaNonTracker::new();
        assert!(bf.vanish_fast(1, &mut vanishing, &mut tracker));
        assert_eq!(expected, vanishing);

        assert_eq!(bf.vanish(1, &mut vanishing, &mut tracker), 40);
        assert_eq!(expected, vanishing);
    }

    #[test]
    fn test_vanish_3() {
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
        let mut tracker = RensaNonTracker::new();
        assert!(!bf.vanish_fast(1, &mut vanishing, &mut tracker));
        assert_eq!(bf.vanish(1, &mut vanishing, &mut tracker), 0);
    }

    #[test]
    fn test_drop_after_vanish_fast() {
        let mut bf = BitField::from_str(concat!(
            "..BB..",
            "RRRR.."));
        let erased = FieldBit::from_str(concat!(
            "1111.."));

        let mut tracker = RensaNonTracker::new();

        let invisible = bf.escape_invisible();
        bf.drop_after_vanish_fast(erased, &mut tracker);
        bf.recover_invisible(&invisible);

        let expected = BitField::from_str(concat!(
            "..BB.."));

        assert_eq!(expected, bf);
    }
}
