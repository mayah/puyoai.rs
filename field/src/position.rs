// TODO(mayah): Is it good to use usize? Should we use i32?
#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
}
