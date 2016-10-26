pub struct RensaResult {
    pub chains: usize,
    pub score: usize,
    pub frames: usize,
    pub quick: bool
}

impl RensaResult {
    pub fn new(chains: usize, score: usize, frames: usize, quick: bool) -> RensaResult {
        RensaResult {
            chains: chains,
            score: score,
            frames: frames,
            quick: quick
        }
    }

    pub fn empty() -> RensaResult {
        RensaResult::new(0, 0, 0, false)
    }
}
