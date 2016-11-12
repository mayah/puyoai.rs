use color::PuyoColor;

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnPuyo {
    x: usize,
    color: PuyoColor,
}

impl ColumnPuyo {
    pub fn new(x: usize, color: PuyoColor) -> ColumnPuyo {
        ColumnPuyo {
            x: x,
            color: color,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn color(&self) -> PuyoColor {
        self.color
    }

    /// Returns true if ColumnPuyo is valid. x should be 0 < x < 7.
    ///
    /// ```
    /// use puyocore::color::PuyoColor;
    /// use puyocore::column_puyo::ColumnPuyo;
    /// let cp0 = ColumnPuyo::new(0, PuyoColor::RED);
    /// let cp1 = ColumnPuyo::new(1, PuyoColor::RED);
    /// let cp6 = ColumnPuyo::new(1, PuyoColor::RED);
    /// let cp7 = ColumnPuyo::new(7, PuyoColor::RED);
    /// assert!(!cp0.valid());
    /// assert!(cp1.valid());
    /// assert!(cp6.valid());
    /// assert!(!cp7.valid());
    /// ```
    pub fn valid(&self) -> bool {
        0 < self.x && self.x < 7
    }
}
