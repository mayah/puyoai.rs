#![feature(simd, simd_ffi)]
#![feature(platform_intrinsics)]

extern crate llvmint;
extern crate simd;

// TODO(mayah): We'd like to make mm_andnot_si128. However, there is no common
// base type for simd vectors. So, we make typed version of functions here.

// TODO(mayah): Use macro?

#[inline]
unsafe fn bitcast<T, U>(x: T) -> U {
    debug_assert!(std::mem::size_of::<T>() == std::mem::size_of::<U>());
    std::mem::transmute_copy(&x)
}

extern "platform-intrinsic" {
    fn simd_shuffle16<S, T>(x: S, y: S, idx: [u32; 16]) -> T;
}

/// Returns !x & y.
/// This will be compiled as "vandnps %xmm1,%xmm0,%xmm0"
pub fn mm_andnot_si128(x: simd::u16x8, y: simd::u16x8) -> simd::u16x8 {
    let z = simd::u16x8::splat(0xFFFF);
    (x ^ z) & y
}

pub fn mm_and_si128(x: simd::u16x8, y: simd::u16x8) -> simd::u16x8 {
    x & y
}

pub fn mm_or_si128(x: simd::u16x8, y: simd::u16x8) -> simd::u16x8 {
    x | y
}

pub fn mm_slli_epi16(x: simd::u16x8, y: i32) -> simd::u16x8 {
    unsafe { bitcast(llvmint::x86::sse2_pslli_w(bitcast(x), y)) }
}

pub fn mm_srli_epi16(x: simd::u16x8, y: i32) -> simd::u16x8 {
    // x
    unsafe { bitcast(llvmint::x86::sse2_psrli_w(bitcast(x), y)) }
}

pub fn mm_slli_si128_1(x: simd::u16x8) -> simd::u16x8 {
    // This is error: shuffle indices are not constant [E0526] ???
    // const idx_1: [u32; 16] = [
    //     16 - 1, 17 - 1, 18 - 1, 19 - 1, 20 - 1, 21 - 1, 22 - 1, 23 - 1,
    //     24 - 1, 25 - 1, 26 - 1, 27 - 1, 28 - 1, 29 - 1, 30 - 1, 31 - 1
    // ];
    // But this is OK. Ugh...
    const IDX: [u32; 16] = [
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30
    ];

    let zero = simd::u8x16::splat(0);
    unsafe {
        let xx: simd::u8x16 = bitcast(x) ;
        let yy: simd::u8x16 = simd_shuffle16(zero, xx, IDX);
        bitcast(yy)
    }
}

pub fn mm_slli_si128_2(x: simd::u16x8) -> simd::u16x8 {
    const IDX: [u32; 16] = [
        14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    ];

    let zero = simd::u8x16::splat(0);
    unsafe {
        let xx: simd::u8x16 = bitcast(x) ;
        let yy: simd::u8x16 = simd_shuffle16(zero, xx, IDX);
        bitcast(yy)
    }
}

pub fn mm_srli_si128_1(x: simd::u16x8) -> simd::u16x8 {
    //const idx_2: [u32; 16] = [
    //    1 + 0, 1 + 1, 1 +  2, 1 +  3, 1 +  4, 1 +  5, 1 +  6, 1 +  7,
    //    1 + 8, 1 + 9, 1 + 10, 1 + 11, 1 + 12, 1 + 13, 1 + 14, 1 + 15
    //];
    const IDX: [u32; 16] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ];

    let zero = simd::u8x16::splat(0);

    unsafe {
        let xx: simd::u8x16 = bitcast(x);
        let yy: simd::u8x16 = simd_shuffle16(xx, zero, IDX);
        bitcast(yy)
    }
}

pub fn mm_srli_si128_2(x: simd::u16x8) -> simd::u16x8 {
    //const idx_2: [u32; 16] = [
    //    1 + 0, 1 + 1, 1 +  2, 1 +  3, 1 +  4, 1 +  5, 1 +  6, 1 +  7,
    //    1 + 8, 1 + 9, 1 + 10, 1 + 11, 1 + 12, 1 + 13, 1 + 14, 1 + 15
    //];
    const IDX: [u32; 16] = [
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
    ];

    let zero = simd::u8x16::splat(0);

    unsafe {
        let xx: simd::u8x16 = bitcast(x);
        let yy: simd::u8x16 = simd_shuffle16(xx, zero, IDX);
        bitcast(yy)
    }
}

pub fn mm_testc_si128(x: simd::u16x8, y: simd::u16x8) -> bool {
    let c = unsafe { llvmint::x86::sse41_ptestc(bitcast(x), bitcast(y)) };
    c != 0
}

#[cfg(test)]
mod tests {
    use simd;
    use super::{bitcast, mm_slli_si128_1, mm_slli_si128_2,
                mm_srli_si128_1, mm_srli_si128_2};

    #[test]
    fn test_mm_slli_si128() {
        let original: simd::u16x8 = unsafe {
            bitcast(simd::u8x16::new(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                     0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10))
        };

        let expected_y1 = simd::u8x16::new(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                                           0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
        let expected_y2 = simd::u8x16::new(0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                                           0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E);

        let y1: simd::u8x16 = unsafe { bitcast(mm_slli_si128_1(original)) };
        let y2: simd::u8x16 = unsafe { bitcast(mm_slli_si128_2(original)) };

        for i in 0 .. 16 {
            assert_eq!(y1.extract(i), expected_y1.extract(i));
            assert_eq!(y2.extract(i), expected_y2.extract(i));
        }
    }

    #[test]
    fn test_mm_slri_si128() {
        let original: simd::u16x8 = unsafe {
            bitcast(simd::u8x16::new(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                                     0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10))
        };

        let expected_y1 = simd::u8x16::new(0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                                           0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x00);
        let expected_y2 = simd::u8x16::new(0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
                                           0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x00, 0x00);

        let y1: simd::u8x16 = unsafe { bitcast(mm_srli_si128_1(original)) };
        let y2: simd::u8x16 = unsafe { bitcast(mm_srli_si128_2(original)) };

        for i in 0 .. 16 {
            assert_eq!(y1.extract(i), expected_y1.extract(i));
            assert_eq!(y2.extract(i), expected_y2.extract(i));
        }
    }
}
