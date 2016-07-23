#![feature(platform_intrinsics)]
#![feature(simd, simd_ffi, link_llvm_intrinsics)]

extern crate simd;

extern crate color;
extern crate simdext;

pub mod bit_field;
pub mod field;
mod field_bit;
pub mod field_checker;
pub mod frame;
pub mod plain_field;
pub mod position;
pub mod rensa;
pub mod score;

pub use plain_field::PuyoPlainField;
pub use plain_field::RealPlainField;
pub use field_checker::FieldChecker;
pub use position::Position;

#[cfg(test)]
mod simulation_tests;
