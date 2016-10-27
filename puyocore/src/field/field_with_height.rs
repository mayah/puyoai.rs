use field::{Field, FieldHeight};

pub struct FieldWithHeight<F: Field> {
    field: F,
    height: [u16; 8],
}

impl<F: Field> FieldWithHeight<F> {
    pub fn new() -> FieldWithHeight<F> {
        FieldWithHeight::<F> {
            field: F::new(),
            height: [0; 8],
        }
    }
}

impl<F: Field> FieldHeight for FieldWithHeight<F> {
    #[inline]
    fn height(&self, x: usize) -> usize {
        self.height[x] as usize
    }
}
