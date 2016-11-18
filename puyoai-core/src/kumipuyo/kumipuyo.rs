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

    pub fn is_rep(&self) -> bool {
        self.axis == self.child
    }

    pub fn valid(&self) -> bool {
        self.axis.is_normal_color() && self.child.is_normal_color()
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

    #[test]
    fn test_is_rep() {
        assert!(Kumipuyo::new(PuyoColor::RED, PuyoColor::RED).is_rep());
        assert!(!Kumipuyo::new(PuyoColor::RED, PuyoColor::BLUE).is_rep());
        assert!(!Kumipuyo::new(PuyoColor::BLUE, PuyoColor::RED).is_rep());
    }

    #[test]
    fn test_valid() {
        assert!(Kumipuyo::new(PuyoColor::RED, PuyoColor::RED).valid());
        assert!(Kumipuyo::new(PuyoColor::RED, PuyoColor::BLUE).valid());
        assert!(Kumipuyo::new(PuyoColor::BLUE, PuyoColor::RED).valid());
        assert!(!Kumipuyo::new(PuyoColor::EMPTY, PuyoColor::RED).valid());
        assert!(!Kumipuyo::new(PuyoColor::RED, PuyoColor::EMPTY).valid());
        assert!(!Kumipuyo::new(PuyoColor::EMPTY, PuyoColor::EMPTY).valid());
    }
}
