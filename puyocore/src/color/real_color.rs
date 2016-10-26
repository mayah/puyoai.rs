use color::Color;
use std;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RealColor {
    EMPTY  = 0,
    WALL   = 1,
    OJAMA  = 2,
    RED    = 3,
    BLUE   = 4,
    YELLOW = 5,
    GREEN  = 6,
    PURPLE = 7,
}

impl RealColor {
    pub fn to_string(&self) -> &'static str {
        match *self {
            RealColor::EMPTY  => "EMPTY",
            RealColor::OJAMA  => "OJAMA",
            RealColor::WALL   => "WALL",
            RealColor::RED    => "RED",
            RealColor::BLUE   => "BLUE",
            RealColor::YELLOW => "YELLOW",
            RealColor::GREEN  => "GREEN",
            RealColor::PURPLE => "PURPLE",
        }
    }
}

impl Color<RealColor> for RealColor {
    fn from_byte(c: u8) -> RealColor {
        match c {
            b' ' | b'.' => RealColor::EMPTY,
            b'O' | b'@' => RealColor::OJAMA,
            b'#' => RealColor::WALL,
            b'R' | b'r' => RealColor::RED,
            b'B' | b'b' => RealColor::BLUE,
            b'Y' | b'y' => RealColor::YELLOW,
            b'G' | b'g' => RealColor::GREEN,
            b'P' | b'p' => RealColor::PURPLE,
            _ => RealColor::EMPTY,

        }
    }

    fn empty_color() -> RealColor {
        RealColor::EMPTY
    }

    fn ojama_color() -> RealColor {
        RealColor::OJAMA
    }

    fn wall_color() -> RealColor {
        RealColor::WALL
    }

    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn is_normal_color(&self) -> bool {
        match *self {
            RealColor::EMPTY  => false,
            RealColor::OJAMA  => false,
            RealColor::WALL   => false,
            RealColor::RED    => true,
            RealColor::BLUE   => true,
            RealColor::YELLOW => true,
            RealColor::GREEN  => true,
            RealColor::PURPLE => true,
        }
    }

    fn to_char(&self) -> char {
        match *self {
            RealColor::EMPTY  => ' ',
            RealColor::OJAMA  => 'O',
            RealColor::WALL   => '#',
            RealColor::RED    => 'R',
            RealColor::BLUE   => 'B',
            RealColor::YELLOW => 'Y',
            RealColor::GREEN  => 'G',
            RealColor::PURPLE => 'P',
        }
    }

    fn as_str(&self) -> &'static str {
        match *self {
            RealColor::EMPTY  => " ",
            RealColor::OJAMA  => "@",
            RealColor::WALL   => "#",
            RealColor::RED    => "R",
            RealColor::BLUE   => "B",
            RealColor::YELLOW => "Y",
            RealColor::GREEN  => "G",
            RealColor::PURPLE => "P",
        }
    }

    fn as_str_wide(&self) -> &'static str {
        match *self {
            RealColor::EMPTY  => "  ",
            RealColor::OJAMA  => "@ ",
            RealColor::WALL   => "# ",
            RealColor::RED    => "R ",
            RealColor::BLUE   => "B ",
            RealColor::YELLOW => "Y ",
            RealColor::GREEN  => "G ",
            RealColor::PURPLE => "P ",
        }
    }

    fn as_colored_str_wide(&self) -> &'static str {
        match *self {
            RealColor::EMPTY  => "  ",
            RealColor::OJAMA  => "@@",
            RealColor::WALL   => "##",
            RealColor::RED    => "\x1b[41m  \x1b[49m",
            RealColor::BLUE   => "\x1b[44m  \x1b[49m",
            RealColor::YELLOW => "\x1b[43m  \x1b[49m",
            RealColor::GREEN  => "\x1b[42m  \x1b[49m",
            RealColor::PURPLE => "\x1b[45m  \x1b[49m",
        }
    }
}

impl std::fmt::Display for RealColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::color::Color;

    #[test]
    fn it_works() {
    }

    #[test]
    fn test_normal_color() {
        assert!(!RealColor::EMPTY.is_normal_color());
        assert!(!RealColor::OJAMA.is_normal_color());
        assert!(!RealColor::WALL.is_normal_color());
        assert!(RealColor::RED.is_normal_color());
        assert!(RealColor::BLUE.is_normal_color());
        assert!(RealColor::YELLOW.is_normal_color());
        assert!(RealColor::GREEN.is_normal_color());
        assert!(RealColor::PURPLE.is_normal_color());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(RealColor::EMPTY.to_string(), "EMPTY");
        assert_eq!(RealColor::OJAMA.to_string(), "OJAMA");
        assert_eq!(RealColor::WALL.to_string(), "WALL");
        assert_eq!(RealColor::RED.to_string(), "RED");
        assert_eq!(RealColor::BLUE.to_string(), "BLUE");
        assert_eq!(RealColor::YELLOW.to_string(), "YELLOW");
        assert_eq!(RealColor::GREEN.to_string(), "GREEN");
        assert_eq!(RealColor::PURPLE.to_string(), "PURPLE");
    }

    #[test]
    fn test_from_byte() {
        assert_eq!(RealColor::from_byte(b' '), RealColor::EMPTY);
        assert_eq!(RealColor::from_byte(b'.'), RealColor::EMPTY);
        assert_eq!(RealColor::from_byte(b'#'), RealColor::WALL);
        assert_eq!(RealColor::from_byte(b'O'), RealColor::OJAMA);
        assert_eq!(RealColor::from_byte(b'R'), RealColor::RED);
        assert_eq!(RealColor::from_byte(b'B'), RealColor::BLUE);
        assert_eq!(RealColor::from_byte(b'Y'), RealColor::YELLOW);
        assert_eq!(RealColor::from_byte(b'G'), RealColor::GREEN);
        assert_eq!(RealColor::from_byte(b'P'), RealColor::PURPLE);
    }
}
