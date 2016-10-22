use field_bit::FieldBit;

pub trait RensaTracker {
    fn track_coef(&mut self, nth_chain: usize, num_erased_puyos: usize, long_bonus_coef: usize, color_bonus_coef: usize);
    fn track_vanish(&mut self, nth_chain: usize, vanished: &FieldBit, ojama_vanished: &FieldBit);
    fn track_drop(&mut self, old_low_bits: u64, old_high_bits: u64, new_low_bits: u64, new_high_bits: u64);
}

pub struct RensaNonTracker {}

impl RensaNonTracker {
    pub fn new() -> RensaNonTracker {
        RensaNonTracker{}
    }
}

impl RensaTracker for RensaNonTracker {
    fn track_coef(&mut self, _nth_chain: usize, _num_erased_puyos: usize, _long_bonus_coef: usize, _color_bonus_coef: usize) {}
    fn track_vanish(&mut self, _nth_chain: usize, _vanished: &FieldBit, _ojama_vanished: &FieldBit) {}
    fn track_drop(&mut self, _old_low_bits: u64, _old_high_bits: u64, _new_low_bits: u64, _new_high_bits: u64) {}
}
