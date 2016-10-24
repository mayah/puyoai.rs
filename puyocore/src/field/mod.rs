pub const WIDTH: usize = 6;
pub const HEIGHT: usize = 12;
pub const MAP_WIDTH: usize = 8;
pub const MAP_HEIGHT: usize = 16;

pub mod bit_field;
pub mod core_field;
pub mod plain_field;

#[cfg(test)]
mod simulation_tests;

pub use self::plain_field::PuyoPlainField;
pub use self::plain_field::RealPlainField;
pub use self::bit_field::BitField;
pub use self::core_field::CoreField;
