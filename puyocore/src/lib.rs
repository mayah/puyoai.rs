#![feature(platform_intrinsics)]
#![feature(simd, simd_ffi, link_llvm_intrinsics)]

extern crate rand;
extern crate x86intrin;

pub mod color;
pub mod decision;
pub mod field;
mod field_bit;
pub mod field_checker;
pub mod frame;
pub mod kumipuyo;
pub mod position;
pub mod rensa_result;
pub mod rensa_tracker;
pub mod score;
pub mod sseext;
