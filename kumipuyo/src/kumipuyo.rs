use color::PuyoColor;

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
