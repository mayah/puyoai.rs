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

    pub fn valid(&self) -> bool {
        0 < self.x && self.x < 7
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::PuyoColor;

    #[test]
    fn test_is_valid() {
        assert!(!ColumnPuyo::new(0, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(1, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(2, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(3, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(4, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(5, PuyoColor::EMPTY).valid());
        assert!(ColumnPuyo::new(6, PuyoColor::EMPTY).valid());
        assert!(!ColumnPuyo::new(7, PuyoColor::EMPTY).valid());
    }
}
