pub trait Color<T> : Clone + Copy + PartialEq<T> {
    fn from_byte(b: u8) -> T;
    fn empty_color() -> T;
    fn ojama_color() -> T;
    fn wall_color() -> T;

    fn as_usize(&self) -> usize;
    fn to_char(&self) -> char;
    fn is_normal_color(&self) -> bool;

    fn as_str(&self) -> &'static str;
    // 2 byte string
    fn as_str_wide(&self) -> &'static str;
    // 2 byte string (with color escape sequence)
    fn as_colored_str_wide(&self) -> &'static str;
}
