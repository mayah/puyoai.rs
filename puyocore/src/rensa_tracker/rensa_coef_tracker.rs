use field_bit::FieldBit;
use rensa_tracker::RensaTracker;

pub struct RensaCoefTracker {
    pub num_erased: [usize; 20],
    pub long_bonus_coef: [usize; 20],
    pub color_bonus_coef: [usize; 20],
}

impl RensaCoefTracker {
    fn new() -> RensaCoefTracker {
        RensaCoefTracker {
            num_erased: [0; 20],
            long_bonus_coef: [0; 20],
            color_bonus_coef: [0; 20],
        }
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
