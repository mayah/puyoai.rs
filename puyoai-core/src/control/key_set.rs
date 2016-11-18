use control::Key;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeySet {
    keys: usize,  // bit flags
}

impl KeySet {
    pub fn new() -> KeySet {
        KeySet {
            keys: 0,
        }
    }

    pub fn from_key(k: Key) -> KeySet {
        let mut ks = KeySet::new();
        ks.set_key(k);
        ks
    }

    pub fn from_keys(keys: &[Key]) -> KeySet {
        let mut ks = KeySet::new();
        for k in keys {
            ks.set_key(*k);
        }
        ks
    }

    pub fn set_key(&mut self, k: Key) {
        self.keys = self.keys | (1 << (k as usize))
    }

    pub fn has_key(&self, k: Key) -> bool {
        (self.keys & (1 << (k as usize))) != 0
    }
}

pub fn parse_keysetseq(s: &str) -> Result<Vec<KeySet>, String> {
    let mut keysetseq = Vec::new();
    for x in s.split(',') {
        let mut ks = KeySet::new();
        for c in x.chars() {
            ks.set_key(try!(Key::parse_char(c)))
        }
        keysetseq.push(ks);
    }
    Ok(keysetseq)
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_parse_keysetseq_1() {
        let expected: &[KeySet] = &[
            KeySet::from_key(Key::Right),
            KeySet::from_key(Key::Left),
            KeySet::from_key(Key::Down),
            KeySet::from_key(Key::Up),
            KeySet::from_key(Key::RightTurn),
            KeySet::from_key(Key::LeftTurn),
            KeySet::from_key(Key::Start),
        ];

        assert_eq!(expected, parse_keysetseq(">,<,v,^,A,B,S").unwrap().as_slice());
    }

    #[test]
    fn test_parse_keysetseq_2() {
        let expected: &[KeySet] = &[
            KeySet::from_keys(&[Key::Right, Key::RightTurn]),
            KeySet::from_keys(&[Key::RightTurn, Key::Right]),
            KeySet::from_keys(&[Key::Left, Key::LeftTurn]),
            KeySet::from_keys(&[Key::LeftTurn, Key::Left]),
            KeySet::from_keys(&[Key::Down, Key::RightTurn]),
            KeySet::from_keys(&[Key::Down, Key::LeftTurn]),
        ];

        assert_eq!(expected, parse_keysetseq(">A,>A,<B,<B,vA,vB").unwrap().as_slice());
    }
}
