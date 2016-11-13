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

    pub fn red(&self) -> usize {
        self.red
    }

    pub fn blue(&self) -> usize {
        self.blue
    }

    pub fn yellow(&self) -> usize {
        self.yellow
    }

    pub fn green(&self) -> usize {
        self.green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let ps = PuyoSet::empty();
        assert_eq!(ps.red(), 0);
        assert_eq!(ps.blue(), 0);
        assert_eq!(ps.yellow(), 0);
        assert_eq!(ps.green(), 0);

        let ps = PuyoSet::new(1, 2, 3, 4);
        assert_eq!(ps.red(), 1);
        assert_eq!(ps.blue(), 2);
        assert_eq!(ps.yellow(), 3);
        assert_eq!(ps.green(), 4);
    }
}
