pub struct Decision {
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
        if self.x <= 0 || 6 < self.x || 4 <= self.r {
            return false;
        }
        if (self.x == 1 && self.r == 3) || (self.x == 6 && self.r == 1) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use decision::Decision;

    #[test]
    fn test_xr() {
        let d10 = Decision::new(3, 0);
        let d11 = Decision::new(3, 1);
        let d12 = Decision::new(3, 2);
        let d13 = Decision::new(3, 3);

        assert_eq!(3, d10.axis_x());
        assert_eq!(3, d10.child_x());
        assert_eq!(0, d10.rot());

        assert_eq!(3, d11.axis_x());
        assert_eq!(4, d11.child_x());
        assert_eq!(1, d11.rot());

        assert_eq!(3, d12.axis_x());
        assert_eq!(3, d12.child_x());
        assert_eq!(2, d12.rot());

        assert_eq!(3, d13.axis_x());
        assert_eq!(2, d13.child_x());
        assert_eq!(3, d13.rot());
    }

    #[test]
    fn test_valid() {
        assert!(Decision::new(1, 0).valid());
        assert!(Decision::new(1, 1).valid());
        assert!(Decision::new(1, 2).valid());
        assert!(!Decision::new(1, 3).valid());
        assert!(!Decision::new(1, 4).valid());

        assert!(Decision::new(2, 0).valid());
        assert!(Decision::new(2, 1).valid());
        assert!(Decision::new(2, 2).valid());
        assert!(Decision::new(2, 3).valid());
        assert!(!Decision::new(2, 4).valid());

        assert!(Decision::new(6, 0).valid());
        assert!(!Decision::new(6, 1).valid());
        assert!(Decision::new(6, 2).valid());
        assert!(Decision::new(6, 3).valid());
        assert!(!Decision::new(6, 4).valid());
    }
}
