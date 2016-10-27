pub trait Field {
    fn new() -> Self;
}

/// FieldHeight trait provides height() method
pub trait FieldHeight {
    fn height(&self, x: usize) -> usize;
}
