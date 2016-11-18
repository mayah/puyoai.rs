use field::{Field, FieldHeight, FieldIsEmpty};

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

    pub fn from_field(field: F) -> FieldWithHeight<F> {
        let mut height = [0; 8];
        field.calculate_height(&mut height);
        FieldWithHeight {
            field: field,
            height: height,
        }
    }

    pub fn field(&self) -> &F {
        &self.field
    }

    pub fn field_mut(&mut self) -> &mut F {
        &mut self.field
    }
}

impl<F: Field + FieldIsEmpty> FieldIsEmpty for FieldWithHeight<F> {
    #[inline]
    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.field.is_empty(x, y)
    }
}

impl<F: Field> FieldHeight for FieldWithHeight<F> {
    #[inline]
    fn height(&self, x: usize) -> usize {
        self.height[x] as usize
    }
}
