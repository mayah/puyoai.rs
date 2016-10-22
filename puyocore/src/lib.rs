#![feature(platform_intrinsics)]
#![feature(simd, simd_ffi, link_llvm_intrinsics)]

extern crate rand;
extern crate x86intrin;

pub mod bit_field;
pub mod color;
pub mod core_field;
pub mod field;
mod field_bit;
pub mod field_checker;
pub mod frame;
pub mod kumipuyo;
pub mod kumipuyo_pos;
pub mod kumipuyo_seq;
pub mod plain_field;
pub mod position;
pub mod rensa;
pub mod score;
pub mod sseext;
pub mod tracker;

pub use plain_field::PuyoPlainField;
pub use plain_field::RealPlainField;
pub use field_checker::FieldChecker;
pub use position::Position;

#[cfg(test)]
mod simulation_tests;
