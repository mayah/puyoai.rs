extern crate simd;

// TODO(mayah): We'd like to make mm_andnot_si128. However, there is no common
// base type for simd vectors. So, we make typed version of functions here.

// TODO(mayah): Use macro?

/// Returns !x & y.
/// This will be compiled as "vandnps %xmm1,%xmm0,%xmm0"
pub fn mm_andnot_epu16(x: simd::u16x8, y: simd::u16x8) -> simd::u16x8 {
    let z = simd::u16x8::splat(0xFFFF);
    (x ^ z) & y
}

pub fn mm_and_epu16(x: simd::u16x8, y: simd::u16x8) -> simd::u16x8 {
    x & y
}
