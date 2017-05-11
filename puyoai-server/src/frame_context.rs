struct FrameContext {
    num_sent_ojama: usize,
    ojama_committed: bool,
}

impl FrameContext {
    pub fn send_ojama(&mut self, num: usize) {
        self.num_sent_ojama += num;
    }

    pub fn commit_ojama(&mut self) {
        self.ojama_committed = true;
    }

    pub fn num_sent_ojama(&self) -> usize {
        self.num_sent_ojama
    }
}
