
struct Decision {
    x: usize,
    r: usize,
}

impl Decision {
    pub fn new(x: usize, r: usize) -> Decision {
        Decision {
            x: x,
            r: r,
        }
    }

    pub fn axis_x(&self) -> usize {
        self.x
    }

    pub fn child_x(&self) -> usize {
        match self.r {
            0 => self.x,
            1 => self.x + 1,
            2 => self.x,
            3 => self.x - 1,
            _ => {
                assert!(false, "unexpected r={}", self.r);
                unreachable!()
            }
        }
    }

    pub fn rot(&self) -> usize {
        self.r
    }

    pub fn valid(&self) -> bool {
        if self.x <= 0 || 6 < self.x || self.r <= 0 || 4 <= self.r {
            return false;
        }
        if (self.x == 1 && self.r == 3) || (self.x == 6 && self.r == 1) {
            return false;
        }

        true
    }
}
