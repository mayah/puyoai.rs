pub struct RensaResult {
    pub chain: usize,
    pub score: usize,
    pub frame: usize,
    pub quick: bool
}

impl RensaResult {
    pub fn new(chain: usize, score: usize, frame: usize, quick: bool) -> RensaResult {
        RensaResult {
            chain: chain,
            score: score,
            frame: frame,
            quick: quick
        }
    }

    pub fn empty() -> RensaResult {
        RensaResult::new(0, 0, 0, false)
    }
}
