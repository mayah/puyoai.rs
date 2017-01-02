use field_bit::FieldBit;
use rensa_tracker::RensaTracker;
use score;

pub struct RensaCoefTracker {
    pub num_erased: [usize; 20],
    pub long_bonus_coef: [usize; 20],
    pub color_bonus_coef: [usize; 20],
}

impl RensaCoefTracker {
    pub fn new() -> RensaCoefTracker {
        RensaCoefTracker {
            num_erased: [0; 20],
            long_bonus_coef: [0; 20],
            color_bonus_coef: [0; 20],
        }
    }

    pub fn coef(&self, nth_chain: usize) -> usize {
        score::calculate_rensa_bonus_coef(score::chain_bonus(nth_chain),
                                          self.long_bonus_coef[nth_chain],
                                          self.color_bonus_coef[nth_chain])
    }
}

impl RensaTracker for RensaCoefTracker {
    fn track_coef(&mut self, nth_chain: usize, num_erased: usize, long_bonus_coef: usize, color_bonus_coef: usize) {
        self.num_erased[nth_chain] = num_erased;
        self.long_bonus_coef[nth_chain] = long_bonus_coef;
        self.color_bonus_coef[nth_chain] = color_bonus_coef;
    }
    fn track_vanish(&mut self, _nth_chain: usize, _vanished: &FieldBit, _ojama_vanished: &FieldBit) {}
    fn track_drop(&mut self, _old_low_bits: u64, _old_high_bits: u64, _new_low_bits: u64, _new_high_bits: u64) {}
}

#[cfg(test)]
mod tests {
    use super::RensaCoefTracker;
    use rensa_tracker::RensaTracker;

    #[test]
    fn test_score() {
        let mut tracker = RensaCoefTracker::new();
        tracker.track_coef(1, 4, 0, 0);
        tracker.track_coef(2, 4, 0, 0);
        tracker.track_coef(3, 4, 0, 0);

        assert_eq!(4, tracker.num_erased[3]);
        assert_eq!(0, tracker.num_erased[4]);
    }
}

#[cfg(all(test, target_feature = "avx2", target_feature="bmi2"))]
mod tests_for_avx2 {
    use super::RensaCoefTracker;
    use field::BitField;

    #[test]
    fn test_simulate() {
        let mut bf = BitField::from_str(concat!(
            "R...RR",
            "RGBRYR",
            "RRGBBY",
            "GGBYYR"));
        let mut tracker = RensaCoefTracker::new();
        let rensa_result = bf.simulate_with_tracker(&mut tracker);

        assert_eq!(5, rensa_result.chain);

        assert_eq!(4, tracker.num_erased[1]);
        assert_eq!(4, tracker.num_erased[2]);
        assert_eq!(4, tracker.num_erased[3]);
        assert_eq!(4, tracker.num_erased[4]);
        assert_eq!(5, tracker.num_erased[5]);

        assert_eq!(1, tracker.coef(1));
        assert_eq!(8, tracker.coef(2));
        assert_eq!(16, tracker.coef(3));
        assert_eq!(32, tracker.coef(4));
        assert_eq!(64 + 2, tracker.coef(5));
    }
}
