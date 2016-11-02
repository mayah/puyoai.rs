use color::Color;

#[derive(Debug, PartialEq)]
pub struct Kumipuyo<C: Color> {
    pub axis: C,
    pub child: C,
}

impl<C: Color> Kumipuyo<C> {
    pub fn new(axis: C, child: C) -> Kumipuyo<C> {
        Kumipuyo {
            axis: axis,
            child: child,
        }
    }

    pub fn axis(&self) -> C {
        self.axis
    }

    pub fn child(&self) -> C {
        self.child
    }
}

#[cfg(test)]
mod tests {
    use super::Kumipuyo;
    use color::PuyoColor;

    #[test]
    fn test_constructor() {
        let kp = Kumipuyo::<PuyoColor>::new(PuyoColor::RED, PuyoColor::BLUE);
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
