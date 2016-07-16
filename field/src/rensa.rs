pub struct RensaResult {
    pub chains: i32,
    pub score: i32,
    pub frames: i32,
    pub quick: bool
}

impl RensaResult {
    pub fn new(chains: i32, score: i32, frames: i32, quick: bool) -> RensaResult {
        RensaResult {
            chains: chains,
            score: score,
            frames: frames,
            quick: quick
        }
    }

    pub fn new_empty() -> RensaResult {
        RensaResult::new(0, 0, 0, false)
    }
}
