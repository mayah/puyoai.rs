pub trait Field {
    fn new() -> Self;

    // Calculates height and sets height to `height`.
    fn calculate_height(&self, height: &mut [u16]);
}

/// FieldHeight trait provides height() method
pub trait FieldHeight {
    fn height(&self, x: usize) -> usize;
}

/// FieldIsEmpty trait provides is_empty() method
pub trait FieldIsEmpty {
    fn is_empty(&self, x: usize, y: usize) -> bool;
}
