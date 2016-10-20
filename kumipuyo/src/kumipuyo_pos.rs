pub struct KumipuyoPos {
    x: i32,
    y: i32,
    r: i32,
}

impl KumipuyoPos {
    pub fn new(x: i32, y: i32, r: i32) -> KumipuyoPos {
        KumipuyoPos {
            x: x,
            y: y,
            r: r,
        }
    }

    pub fn initial_pos() -> KumipuyoPos {
        return KumipuyoPos::new(3, 12, 0)
    }

    pub fn axis_x(&self) -> i32 {
        self.x
    }

    pub fn axis_y(&self) -> i32 {
        self.y
    }

    pub fn child_x(&self) -> i32 {
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

    pub fn child_y(&self) -> i32 {
        match self.r {
            0 => self.y + 1,
            1 => self.y,
            2 => self.y - 1,
            3 => self.y,
            _ => {
                assert!(false, "unexpected r={}", self.r);
                unreachable!()
            }
        }
    }

    pub fn rot(&self) -> i32 {
        self.r
    }

    pub fn valid(&self) -> bool {
        if self.y < 1 || 13 < self.y {
            return false;
        }
        if self.x < 1 || 6 < self.x {
            return false;
        }
        if self.r < 0 || 4 < self.r {
            return false;
        }
        if self.x == 1 && self.r == 3 {
            return false;
        }
        if self.x == 6 && self.r == 1 {
            return false;
        }
        true
    }
}
