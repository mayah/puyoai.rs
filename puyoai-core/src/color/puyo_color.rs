use color::Color;
use std;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PuyoColor {
    EMPTY  = 0,
    OJAMA  = 1,
    WALL   = 2,
    IRON   = 3,
    RED    = 4,
    BLUE   = 5,
    YELLOW = 6,
    GREEN  = 7
}

pub const NUM_PUYO_COLORS: usize = 8;

const ALL_PUYO_COLORS: &'static [PuyoColor] = &[
    PuyoColor::EMPTY, PuyoColor::OJAMA, PuyoColor::WALL, PuyoColor::IRON,
    PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN,
];

const ALL_NORMAL_PUYO_COLORS: &'static [PuyoColor] = &[
    PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN,
];

impl PuyoColor {
    pub fn to_string(&self) -> &'static str {
        match *self {
            PuyoColor::EMPTY  => "EMPTY",
            PuyoColor::OJAMA  => "OJAMA",
            PuyoColor::WALL   => "WALL",
            PuyoColor::IRON   => "IRON",
            PuyoColor::RED    => "RED",
            PuyoColor::BLUE   => "BLUE",
            PuyoColor::YELLOW => "YELLOW",
            PuyoColor::GREEN  => "GREEN",
        }
    }
}

impl Color for PuyoColor {
    fn from_byte(c: u8) -> PuyoColor {
        match c {
            b' ' | b'.' => PuyoColor::EMPTY,
            b'O' | b'@' => PuyoColor::OJAMA,
            b'#' => PuyoColor::WALL,
            b'&' => PuyoColor::IRON,
            b'R' | b'r' => PuyoColor::RED,
            b'B' | b'b' => PuyoColor::BLUE,
            b'Y' | b'y' => PuyoColor::YELLOW,
            b'G' | b'g' => PuyoColor::GREEN,
            _ => PuyoColor::EMPTY,
        }
    }

    fn empty_color() -> PuyoColor {
        PuyoColor::EMPTY
    }

    fn ojama_color() -> PuyoColor {
        PuyoColor::OJAMA
    }

    fn wall_color() -> PuyoColor {
        PuyoColor::WALL
    }

    fn all_normal_colors() -> &'static [PuyoColor] {
        ALL_NORMAL_PUYO_COLORS
    }

    fn all_colors() -> &'static [PuyoColor] {
        ALL_PUYO_COLORS
    }

    fn as_usize(&self) -> usize {
        *self as usize
    }

    fn is_normal_color(&self) -> bool {
        let x = *self as i32;
        (x & 4) != 0
    }

    fn to_char(&self) -> char {
        match *self {
            PuyoColor::EMPTY  => ' ',
            PuyoColor::OJAMA  => 'O',
            PuyoColor::WALL   => '#',
            PuyoColor::IRON   => '&',
            PuyoColor::RED    => 'R',
            PuyoColor::BLUE   => 'B',
            PuyoColor::YELLOW => 'Y',
            PuyoColor::GREEN  => 'G',
        }
    }

    fn as_str(&self) -> &'static str {
        match *self {
            PuyoColor::EMPTY  => " ",
            PuyoColor::OJAMA  => "@",
            PuyoColor::WALL   => "#",
            PuyoColor::IRON   => "&",
            PuyoColor::RED    => "R",
            PuyoColor::BLUE   => "B",
            PuyoColor::YELLOW => "Y",
            PuyoColor::GREEN  => "G",
        }
    }

    fn as_str_wide(&self) -> &'static str {
        match *self {
            PuyoColor::EMPTY  => "  ",
            PuyoColor::OJAMA  => "@ ",
            PuyoColor::WALL   => "# ",
            PuyoColor::IRON   => "& ",
            PuyoColor::RED    => "R ",
            PuyoColor::BLUE   => "B ",
            PuyoColor::YELLOW => "Y ",
            PuyoColor::GREEN  => "G ",
        }
    }

    fn as_colored_str_wide(&self) -> &'static str {
        match *self {
            PuyoColor::EMPTY  => "  ",
            PuyoColor::OJAMA  => "@@",
            PuyoColor::WALL   => "##",
            PuyoColor::IRON   => "&&",
            PuyoColor::RED    => "\x1b[41m  \x1b[49m",
            PuyoColor::BLUE   => "\x1b[44m  \x1b[49m",
            PuyoColor::YELLOW => "\x1b[43m  \x1b[49m",
            PuyoColor::GREEN  => "\x1b[42m  \x1b[49m",
        }
    }
}

impl std::fmt::Display for PuyoColor {
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
        assert!(!PuyoColor::EMPTY.is_normal_color());
        assert!(!PuyoColor::OJAMA.is_normal_color());
        assert!(!PuyoColor::WALL.is_normal_color());
        assert!(!PuyoColor::IRON.is_normal_color());
        assert!(PuyoColor::RED.is_normal_color());
        assert!(PuyoColor::BLUE.is_normal_color());
        assert!(PuyoColor::YELLOW.is_normal_color());
        assert!(PuyoColor::GREEN.is_normal_color());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(PuyoColor::EMPTY.to_string(), "EMPTY");
        assert_eq!(PuyoColor::OJAMA.to_string(), "OJAMA");
        assert_eq!(PuyoColor::WALL.to_string(), "WALL");
        assert_eq!(PuyoColor::IRON.to_string(), "IRON");
        assert_eq!(PuyoColor::RED.to_string(), "RED");
        assert_eq!(PuyoColor::BLUE.to_string(), "BLUE");
        assert_eq!(PuyoColor::YELLOW.to_string(), "YELLOW");
        assert_eq!(PuyoColor::GREEN.to_string(), "GREEN");
    }

    #[test]
    fn test_from_byte() {
        assert_eq!(PuyoColor::from_byte(b' '), PuyoColor::EMPTY);
        assert_eq!(PuyoColor::from_byte(b'.'), PuyoColor::EMPTY);
        assert_eq!(PuyoColor::from_byte(b'&'), PuyoColor::IRON);
        assert_eq!(PuyoColor::from_byte(b'#'), PuyoColor::WALL);
        assert_eq!(PuyoColor::from_byte(b'O'), PuyoColor::OJAMA);
        assert_eq!(PuyoColor::from_byte(b'R'), PuyoColor::RED);
        assert_eq!(PuyoColor::from_byte(b'B'), PuyoColor::BLUE);
        assert_eq!(PuyoColor::from_byte(b'Y'), PuyoColor::YELLOW);
        assert_eq!(PuyoColor::from_byte(b'G'), PuyoColor::GREEN);
    }
}
