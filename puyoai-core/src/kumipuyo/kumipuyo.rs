use color::{self, Color};

// Pair is a pair of falling puyo.
#[derive(Clone, Debug, PartialEq)]
pub struct Pair<C: Color> {
    axis: C,
    child: C,
}

impl<C: Color> Pair<C> {
    pub fn new(axis: C, child: C) -> Pair<C> {
        Pair {
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

/// A pair of PuyoColor.
pub type Kumipuyo = Pair<color::PuyoColor>;

/// A pair of RealColor.
pub type Kumireal = Pair<color::RealColor>;

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
