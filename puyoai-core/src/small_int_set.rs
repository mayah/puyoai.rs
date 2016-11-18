/// SmallIntSet is an integer set that can contain [0, 64).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SmallIntSet {
    v: u64
}

impl SmallIntSet {
    pub fn new() -> SmallIntSet {
        SmallIntSet {
            v: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.v == 0
    }

    pub fn size(&self) -> usize {
        self.v.count_ones() as usize
    }

    pub fn smallest(&self) -> usize {
        debug_assert!(!self.is_empty());
        self.v.trailing_zeros() as usize
    }

    pub fn remove_smallest(&mut self) {
        self.v = self.v & (self.v - 1)
    }

    pub fn is_set(&self, x: usize) -> bool {
        debug_assert!(x < 64);
        (self.v & (1 << x)) != 0
    }


    pub fn set(&mut self, x: usize) {
        debug_assert!(x < 64);
        self.v |= 1 << x
    }

    pub fn unset(&mut self, x: usize) {
        debug_assert!(x < 64);
        self.v &= !(1 << x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut s = SmallIntSet::new();
        assert!(s.is_empty());
        assert_eq!(0, s.size());

        s.set(0);
        s.set(1);
        s.set(5);
        s.set(8);
        s.set(16);
        s.set(31);
        s.set(31);

        assert_eq!(6, s.size());
        s.unset(8);
        assert_eq!(5, s.size());
        assert_eq!(0, s.smallest());
        s.remove_smallest();
        assert_eq!(1, s.smallest());
        s.remove_smallest();
        assert_eq!(5, s.smallest());
        s.remove_smallest();
        assert_eq!(16, s.smallest());
        s.remove_smallest();
        assert_eq!(31, s.smallest());
        s.remove_smallest();
        assert!(s.is_empty());
        assert_eq!(0, s.size());
    }
}
