pub struct PuyoSet {
    red: usize,
    blue: usize,
    yellow: usize,
    green: usize,
}

impl PuyoSet {
    pub fn new(red: usize, blue: usize, yellow: usize, green: usize) -> PuyoSet {
        PuyoSet {
            red: red,
            blue: blue,
            yellow: yellow,
            green: green,
        }
    }

    pub fn empty() -> PuyoSet {
        PuyoSet::new(0, 0, 0, 0)
    }
}
