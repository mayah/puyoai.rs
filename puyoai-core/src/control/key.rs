#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    Up,
    Right,
    Down,
    Left,
    RightTurn,
    LeftTurn,
    Start,
}

impl Key {
    pub fn parse_char(c: char) -> Result<Key, String> {
        match c {
            '^' => Ok(Key::Up),
            '>' => Ok(Key::Right),
            'v' => Ok(Key::Down),
            '<' => Ok(Key::Left),
            'A' => Ok(Key::RightTurn),
            'B' => Ok(Key::LeftTurn),
            'S' => Ok(Key::Start),
            _ => Err(format!("Unknown key character: {}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Key;

    #[test]
    fn test_parse_char() {
        assert_eq!(Key::Up, Key::parse_char('^').unwrap());
        assert_eq!(Key::Right, Key::parse_char('>').unwrap());
        assert_eq!(Key::Down, Key::parse_char('v').unwrap());
        assert_eq!(Key::Left, Key::parse_char('<').unwrap());
        assert_eq!(Key::RightTurn, Key::parse_char('A').unwrap());
        assert_eq!(Key::LeftTurn, Key::parse_char('B').unwrap());
        assert_eq!(Key::Start, Key::parse_char('S').unwrap());

        assert!(Key::parse_char('_').is_err());
    }
}
