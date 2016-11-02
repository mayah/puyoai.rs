pub trait Color : Clone + Copy + PartialEq<Self> {
    fn from_byte(b: u8) -> Self;
    fn empty_color() -> Self;
    fn ojama_color() -> Self;
    fn wall_color() -> Self;

    fn all_colors() -> &'static [Self];
    fn all_normal_colors() -> &'static [Self];

    fn as_usize(&self) -> usize;
    fn to_char(&self) -> char;
    fn is_normal_color(&self) -> bool;

    fn as_str(&self) -> &'static str;
    // 2 byte string
    fn as_str_wide(&self) -> &'static str;
    // 2 byte string (with color escape sequence)
    fn as_colored_str_wide(&self) -> &'static str;
}
