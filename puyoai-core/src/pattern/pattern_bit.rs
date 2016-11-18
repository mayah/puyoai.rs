use field_bit::FieldBit;

pub struct PatternBit {
    var_bit: FieldBit,
    not_bit: FieldBit,
}

impl PatternBit {
    pub fn new(var_bit: FieldBit, not_bit: FieldBit) -> PatternBit {
        PatternBit {
            var_bit: var_bit,
            not_bit: not_bit,
        }
    }

    pub fn var_bit(&self) -> FieldBit {
        self.var_bit
    }

    pub fn not_bit(&self) -> FieldBit {
        self.not_bit
    }
}
