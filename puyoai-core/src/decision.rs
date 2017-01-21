#[derive(Clone, Debug, Eq, PartialOrd, Ord, PartialEq)]
pub struct Decision {
    x: usize,
    r: usize,
}

const ALL_VALID_DECISIONS: &'static [Decision] = &[
    Decision { x: 2, r: 3 },
    Decision { x: 3, r: 3 },
    Decision { x: 3, r: 1 },
    Decision { x: 4, r: 1 },
    Decision { x: 5, r: 1 },
    Decision { x: 1, r: 2 },
    Decision { x: 2, r: 2 },
    Decision { x: 3, r: 2 },
    Decision { x: 4, r: 2 },
    Decision { x: 5, r: 2 },
    Decision { x: 6, r: 2 },
    Decision { x: 1, r: 1 },
    Decision { x: 2, r: 1 },
    Decision { x: 4, r: 3 },
    Decision { x: 5, r: 3 },
    Decision { x: 6, r: 3 },
    Decision { x: 1, r: 0 },
    Decision { x: 2, r: 0 },
    Decision { x: 3, r: 0 },
    Decision { x: 4, r: 0 },
    Decision { x: 5, r: 0 },
    Decision { x: 6, r: 0 },
];

const ALL_VALID_DECISIONS_FOR_REP: &'static [Decision] = &[
    Decision { x: 2, r: 3 },
    Decision { x: 3, r: 3 },
    Decision { x: 3, r: 1 },
    Decision { x: 4, r: 1 },
    Decision { x: 5, r: 1 },
    Decision { x: 1, r: 2 },
    Decision { x: 2, r: 2 },
    Decision { x: 3, r: 2 },
    Decision { x: 4, r: 2 },
    Decision { x: 5, r: 2 },
    Decision { x: 6, r: 2 },
];

impl Decision {
    pub fn new(x: usize, r: usize) -> Decision {
        Decision {
            x: x,
            r: r,
        }
    }

    pub fn all_valid_decisions() -> &'static [Decision] {
        ALL_VALID_DECISIONS
    }

    pub fn all_valid_decisions_for_rep() -> &'static [Decision] {
        ALL_VALID_DECISIONS_FOR_REP
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

    pub fn is_valid(&self) -> bool {
        if self.x <= 0 || 6 < self.x || 4 <= self.r {
            return false;
        }
        if (self.x == 1 && self.r == 3) || (self.x == 6 && self.r == 1) {
            return false;
        }

        true
    }

    pub fn reverse(&self) -> Decision {
        Decision::new(self.child_x(), (self.rot() + 2) & 0x3)
    }
}

#[cfg(test)]
mod tests {
    use decision::Decision;

    #[test]
    fn test_all_valid_decisions() {
        assert_eq!(22, Decision::all_valid_decisions().len());
    }

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
        assert!(Decision::new(1, 0).is_valid());
        assert!(Decision::new(1, 1).is_valid());
        assert!(Decision::new(1, 2).is_valid());
        assert!(!Decision::new(1, 3).is_valid());
        assert!(!Decision::new(1, 4).is_valid());

        assert!(Decision::new(2, 0).is_valid());
        assert!(Decision::new(2, 1).is_valid());
        assert!(Decision::new(2, 2).is_valid());
        assert!(Decision::new(2, 3).is_valid());
        assert!(!Decision::new(2, 4).is_valid());

        assert!(Decision::new(6, 0).is_valid());
        assert!(!Decision::new(6, 1).is_valid());
        assert!(Decision::new(6, 2).is_valid());
        assert!(Decision::new(6, 3).is_valid());
        assert!(!Decision::new(6, 4).is_valid());
    }

    #[test]
    fn test_reverse() {
        assert_eq!(Decision::new(3, 2), Decision::new(3, 0).reverse());
        assert_eq!(Decision::new(4, 3), Decision::new(3, 1).reverse());
        assert_eq!(Decision::new(3, 0), Decision::new(3, 2).reverse());
        assert_eq!(Decision::new(2, 1), Decision::new(3, 3).reverse());
    }

    #[test]
    fn test_decisions_for_rep() {
        let mut expected = Vec::<Decision>::new();
        expected.extend_from_slice(Decision::all_valid_decisions());
        expected.sort();

        assert_eq!(22, expected.len());

        let mut actual = Vec::<Decision>::new();
        actual.extend_from_slice(Decision::all_valid_decisions_for_rep());
        assert_eq!(11, actual.len());
        for d in Decision::all_valid_decisions_for_rep() {
            actual.push(d.reverse());
        }
        actual.sort();
        assert_eq!(22, actual.len());

        assert_eq!(actual, expected);
    }
}
