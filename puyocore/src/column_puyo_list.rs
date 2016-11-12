use color::PuyoColor;
use small_int_set::SmallIntSet;

const MAX_SIZE: usize = 8;

pub struct ColumnPuyoList {
    size: [usize; 6],
    puyo: [[PuyoColor; MAX_SIZE]; 6],
    place_holders: [SmallIntSet; 6],
}

impl ColumnPuyoList {
    pub fn new() -> ColumnPuyoList {
        ColumnPuyoList {
            size: [0; 6],
            puyo: [[PuyoColor::EMPTY; MAX_SIZE]; 6],
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

    pub fn get(&self, x: usize, i: usize) -> PuyoColor {
        debug_assert!(i < self.size_on(x));
        self.puyo[x - 1][i]
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

    pub fn merge(&mut self, cpl: &ColumnPuyoList) -> bool {
        for i in 0..6 {
            if cpl.size[i] >= self.place_holders[i].size() && MAX_SIZE < self.size[i] + (cpl.size[i] - self.place_holders[i].size()) {
                return false;
            }
        }

        for i in 0..6 {
            if cpl.size[i] < self.place_holders[i].size() {
                let discard = self.place_holders[i].size() - cpl.size[i];
                for _ in 0..discard {
                    self.place_holders[i].remove_smallest();
                }
                for j in 0..cpl.size[i] {
                    let k = self.place_holders[i].smallest();
                    self.place_holders[i].remove_smallest();
                    self.puyo[i][k] = cpl.puyo[i][j];
                }
            } else {
                let mut j = 0;
                while !self.place_holders[i].is_empty() {
                    let k = self.place_holders[i].smallest();
                    self.place_holders[i].remove_smallest();
                    self.puyo[i][k] = cpl.puyo[i][j];
                    j += 1;
                }

                while j < cpl.size[i] {
                    self.puyo[i][self.size[i]] = cpl.puyo[i][j];
                    self.size[i] += 1;
                    j += 1;
                }
            }

            let mut new_place_holder = SmallIntSet::new();
            for j in 0..self.size[i] {
                if ColumnPuyoList::is_place_holder(cpl.puyo[i][j]) {
                    new_place_holder.set(j);
                }
            }
            self.place_holders[i] = new_place_holder;
        }

        true
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

    #[test]
    fn test_merge() {
        let mut cpl = ColumnPuyoList::new();
        let mut cpl1 = ColumnPuyoList::new();
        assert!(cpl1.add_multi(3, PuyoColor::RED, 2));
        let mut cpl2 = ColumnPuyoList::new();
        assert!(cpl2.add_multi(3, PuyoColor::BLUE, 2));
        let mut cpl3 = ColumnPuyoList::new();
        assert!(cpl3.add_multi(3, PuyoColor::BLUE, 8));

        assert!(cpl.merge(&cpl1));
        assert!(cpl.merge(&cpl2));
        assert_eq!(4, cpl.size());

        assert!(!cpl.merge(&cpl3));
        assert_eq!(4, cpl.size());
    }

    #[test]
    fn test_merge_with_place_holders_1() {
        let mut cpl1 = ColumnPuyoList::new();
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));

        let mut cpl2 = ColumnPuyoList::new();
        assert!(cpl2.add(3, PuyoColor::IRON));
        assert!(cpl2.add(3, PuyoColor::IRON));
        assert!(cpl2.add(3, PuyoColor::YELLOW));
        assert!(cpl2.add(3, PuyoColor::YELLOW));

        assert!(cpl1.merge(&cpl2));

        assert_eq!(7, cpl1.size());
        assert_eq!(7, cpl1.size_on(3));
        assert_eq!(PuyoColor::IRON, cpl1.get(3, 0));
        assert_eq!(PuyoColor::IRON, cpl1.get(3, 1));
        assert_eq!(PuyoColor::YELLOW, cpl1.get(3, 2));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 3));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 4));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 5));
        assert_eq!(PuyoColor::YELLOW, cpl1.get(3, 6));
    }

    #[test]
    fn test_merge_with_place_holders_2() {
        let mut cpl1 = ColumnPuyoList::new();
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));

        let mut cpl2 = ColumnPuyoList::new();
        assert!(cpl2.add(3, PuyoColor::YELLOW));
        assert!(cpl2.add(3, PuyoColor::YELLOW));

        assert!(cpl1.merge(&cpl2));

        assert_eq!(6, cpl1.size());
        assert_eq!(6, cpl1.size_on(3));
        assert_eq!(PuyoColor::IRON, cpl1.get(3, 0));
        assert_eq!(PuyoColor::YELLOW, cpl1.get(3, 1));
        assert_eq!(PuyoColor::YELLOW, cpl1.get(3, 2));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 3));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 4));
        assert_eq!(PuyoColor::RED, cpl1.get(3, 5));
    }

    #[test]
    fn test_merge_with_place_holders_3() {
        let mut cpl1 = ColumnPuyoList::new();
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::IRON));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));
        assert!(cpl1.add(3, PuyoColor::RED));

        let mut cpl2 = ColumnPuyoList::new();
        assert!(cpl2.add(3, PuyoColor::YELLOW));
        assert!(cpl2.add(3, PuyoColor::YELLOW));
        assert!(cpl2.add(3, PuyoColor::YELLOW));
        assert!(cpl2.add(3, PuyoColor::YELLOW));

        assert!(!cpl1.merge(&cpl2));
    }
}
