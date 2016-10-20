pub trait Color<T> : Clone + Copy + PartialEq<T> {
    fn from_byte(b: u8) -> T;
    fn empty_color() -> T;
    fn ojama_color() -> T;
    fn wall_color() -> T;

    fn as_usize(&self) -> usize;
    fn to_char(&self) -> char;
    fn is_normal_color(&self) -> bool;
}

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

impl Color<PuyoColor> for PuyoColor {
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
}

pub const ALL_PUYO_COLORS: [PuyoColor; 8] = [
    PuyoColor::EMPTY, PuyoColor::OJAMA, PuyoColor::WALL, PuyoColor::IRON,
    PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN,
];

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_to_char() {
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

#[derive(Clone, Copy, PartialEq)]
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
