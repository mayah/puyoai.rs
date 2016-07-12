pub struct FieldChecker {
    flag: [[bool; 16]; 8]
}

impl FieldChecker {
    pub fn new() -> FieldChecker {
        FieldChecker {
            flag: [[false; 16]; 8],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.flag[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.flag[x][y] = true;
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        self.flag[x][y] = false;
    }

    pub fn set_flag(&mut self, x: usize, y: usize, v: bool) {
        self.flag[x][y] = v;
    }
}

#[cfg(test)]
mod tests {
    use field_checker::FieldChecker;

    #[test]
    fn test_initialize() {
        let checker = FieldChecker::new();
        for x in 0..8 {
            for y in 0..16 {
                assert!(!checker.get(x, y));
            }
        }
    }

    #[test]
    fn test_get_and_set() {
        let mut checker = FieldChecker::new();
        for x in 0..8 {
            for y in 0..16 {
                checker.set(x, y);
                assert!(checker.get(x, y));
                checker.unset(x, y);
                assert!(!checker.get(x, y));
                checker.set_flag(x, y, true);
                assert!(checker.get(x, y));
                checker.set_flag(x, y, false);
                assert!(!checker.get(x, y));
            }
        }

    }
}
