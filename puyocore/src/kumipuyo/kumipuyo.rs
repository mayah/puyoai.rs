use color::PuyoColor;

#[derive(Debug, PartialEq)]
pub struct Kumipuyo {
    pub axis: PuyoColor,
    pub child: PuyoColor,
}

impl Kumipuyo {
    pub fn new(axis: PuyoColor, child: PuyoColor) -> Kumipuyo {
        Kumipuyo {
            axis: axis,
            child: child,
        }
    }

    pub fn axis(&self) -> PuyoColor {
        self.axis
    }

    pub fn child(&self) -> PuyoColor {
        self.child
    }
}

#[cfg(test)]
mod tests {
    use super::Kumipuyo;
    use color::PuyoColor;

    #[test]
    fn test_constructor() {
        let kp = Kumipuyo::new(PuyoColor::RED, PuyoColor::BLUE);
        assert_eq!(PuyoColor::RED, kp.axis());
        assert_eq!(PuyoColor::BLUE, kp.child());
    }

    #[test]
    fn test_equal() {
        let kp1 = Kumipuyo::new(PuyoColor::RED, PuyoColor::BLUE);
        let kp2 = Kumipuyo::new(PuyoColor::RED, PuyoColor::BLUE);
        assert_eq!(kp1, kp2);
    }
}
