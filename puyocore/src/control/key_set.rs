use control::Key;

pub struct KeySet {
    keys: usize,  // bit flags
}

impl KeySet {
    pub fn new() -> KeySet {
        KeySet {
            keys: 0,
        }
    }

    pub fn set_key(&mut self, k: Key) {
        self.keys = self.keys | (1 << (k as usize))
    }

    pub fn has_key(&self, k: Key) -> bool {
        (self.keys & (1 << (k as usize))) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::KeySet;
    use control::Key;

    #[test]
    fn test_set_key_has_key() {
        let mut ks = KeySet::new();
        assert!(!ks.has_key(Key::Up));
        assert!(!ks.has_key(Key::Right));
        assert!(!ks.has_key(Key::Left));
        assert!(!ks.has_key(Key::Down));
        assert!(!ks.has_key(Key::RightTurn));
        assert!(!ks.has_key(Key::LeftTurn));
        assert!(!ks.has_key(Key::Start));

        ks.set_key(Key::Up);
        ks.set_key(Key::Right);
        assert!(ks.has_key(Key::Up));
        assert!(ks.has_key(Key::Right));
        assert!(!ks.has_key(Key::Left));
        assert!(!ks.has_key(Key::Down));
        assert!(!ks.has_key(Key::RightTurn));
        assert!(!ks.has_key(Key::LeftTurn));
        assert!(!ks.has_key(Key::Start));
    }
}
