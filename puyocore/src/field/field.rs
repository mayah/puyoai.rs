pub trait Field {
    fn new() -> Self;
}

/// FieldHeight trait provides height() method
pub trait FieldHeight {
    fn height(&self, x: usize) -> usize;
}

/// FieldIsEmpty trait provides is_empty() method
pub trait FieldIsEmpty {
    fn is_empty(&self, x: usize, y: usize) -> bool;
}
