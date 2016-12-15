pub const WIDTH: usize = 6;
pub const HEIGHT: usize = 12;
pub const MAP_WIDTH: usize = 8;
pub const MAP_HEIGHT: usize = 16;

pub mod field;
pub mod field_with_height;
pub mod plain_field;

pub use self::plain_field::PuyoPlainField;
pub use self::plain_field::RealPlainField;
pub use self::field::Field;
pub use self::field::FieldHeight;
pub use self::field::FieldIsEmpty;
pub use self::field_with_height::FieldWithHeight;

pub mod bit_field;
pub mod core_field;

pub use self::bit_field::BitField;
pub use self::core_field::CoreField;

#[cfg(test)]
mod simulation_tests;
