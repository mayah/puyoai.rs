use color::PuyoColor;
use small_int_set::SmallIntSet;

const MAX_SIZE: usize = 8;

pub struct ColumnPuyoList {
    size: [usize; 6],
    puyo: [[PuyoColor; 6]; MAX_SIZE],
    place_holders: [SmallIntSet; 6],
}

impl ColumnPuyoList {
    pub fn new() -> ColumnPuyoList {
        ColumnPuyoList {
            size: [0; 6],
            puyo: [[PuyoColor::EMPTY; 6]; MAX_SIZE],
            place_holders: [SmallIntSet::new(); 6],
        }
    }

    fn is_place_holder(c: PuyoColor) -> bool {
        c == PuyoColor::IRON
    }

    pub fn size_on(&self, x: usize) -> usize {
        self.size[x - 1]
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        self.size[0] + self.size[1] + self.size[2] +
        self.size[3] + self.size[4] + self.size[5]
    }

    pub fn top(&self, x: usize) -> Option<PuyoColor> {
        if self.size_on(x) == 0 {
            None
        } else {
            Some(self.puyo[x - 1][self.size_on(x) - 1])
        }
    }

    pub fn has_place_holder(&self) -> bool {
        for i in 0..6 {
            if !self.place_holders[i].is_empty() {
                return true;
            }
        }

        false
    }

    /// Adds PuyoColor `c` on column `x`.
    /// Returns true if succeeded. Returns false otherwise.
    /// When failed, the `self` won't be changed.
    pub fn add(&mut self, x: usize, c: PuyoColor) -> bool {
        if MAX_SIZE <= self.size[x - 1] {
            return false;
        }
        if ColumnPuyoList::is_place_holder(c) {
            self.place_holders[x - 1].set(self.size[x - 1]);
        }
        self.puyo[x - 1][self.size[x - 1]] = c;
        self.size[x - 1] += 1;
        true
    }

    /// Adds `n` PuyoColor `c` on column `x`.
    /// Returns true if succeeded. Returns false otherwise.
    /// When failed, the `self` won't be changed.
    pub fn add_multi(&mut self, x: usize, c: PuyoColor, n: usize) -> bool {
        if MAX_SIZE < self.size_on(x) + n {
            return false;
        }
        for i in 0..n {
            if ColumnPuyoList::is_place_holder(c) {
                self.place_holders[x - 1].set(self.size[x - 1] + i);
            }
            self.puyo[x - 1][self.size[x - 1] + i] = c;
        }
        self.size[x - 1] += n;
        true
    }

    /// Removes top puyo from column x.
    pub fn remove_top(&mut self, x: usize) {
        if self.size_on(x) == 0 {
            return;
        }
        let c = self.puyo[x - 1][self.size[x - 1] - 1];
        if ColumnPuyoList::is_place_holder(c) {
            self.place_holders[x - 1].unset(self.size[x - 1] - 1);
        }
        self.size[x - 1] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::PuyoColor;

    #[test]
    fn test_constructor() {
        let cpl = ColumnPuyoList::new();
        assert_eq!(cpl.size(), 0);
        assert!(cpl.is_empty());
    }

    #[test]
    fn test_add() {
        let mut cpl = ColumnPuyoList::new();
        cpl.add(1, PuyoColor::RED);
        cpl.add(1, PuyoColor::BLUE);
        cpl.add(2, PuyoColor::YELLOW);

        assert_eq!(cpl.size_on(1), 2);
        assert_eq!(cpl.size_on(2), 1);
        assert_eq!(cpl.size_on(3), 0);
        assert_eq!(cpl.size_on(4), 0);
        assert_eq!(cpl.size_on(5), 0);
        assert_eq!(cpl.size_on(6), 0);

        assert_eq!(cpl.top(1), Some(PuyoColor::BLUE));
        assert_eq!(cpl.top(2), Some(PuyoColor::YELLOW));
        assert_eq!(cpl.top(3), None);
    }

    #[test]
    fn test_place_holder() {
        let mut cpl = ColumnPuyoList::new();
        assert!(!cpl.has_place_holder());

        cpl.add(1, PuyoColor::RED);
        assert!(!cpl.has_place_holder());

        cpl.add(1, PuyoColor::IRON);
        assert!(cpl.has_place_holder());

        cpl.remove_top(1);
        assert!(!cpl.has_place_holder());
    }
}
