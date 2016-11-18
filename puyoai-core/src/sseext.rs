use x86intrin::*;

/// Returns __m128i where all bits are set to 1.
#[inline]
pub fn mm_setone_si128() -> m128i {
    let zero = mm_setzero_si128();
    mm_cmpeq_epi64(zero, zero)
}

/// Bit-wise not for __m128i.
#[inline]
pub fn mm_not_si128(a: m128i) -> m128i {
    mm_setone_si128() ^ a
}

/// Parallel bit-wise or operation for each 16 bits.
/// 0001xxxxxxxxxxxx --> 0001111111111111
#[inline]
pub fn mm_porr_epi16(mut a: m128i) -> m128i {
    a = a | mm_srli_epi16(a, 1);
    a = a | mm_srli_epi16(a, 2);
    a = a | mm_srli_epi16(a, 4);
    a = a | mm_srli_epi16(a, 8);
    return a;
}

/// Returns the max value for each 16-bit values.
#[inline]
pub fn mm_hmax_epu16(a: m128i) -> u16 {
    // Unfortunately, there is no _mm_maxpos_epu16 builtin API.
    // Instead, use _mm_minpos_epu16 with negating the bits.
    let not_maxpos = mm_minpos_epu16(mm_not_si128(a));
    return ((!mm_cvtsi128_si32(not_maxpos)) & 0xFFFF) as u16;
}

/// popcount 8 x 16bits
#[inline]
pub fn mm_popcnt_epi16(x: m128i) -> m128i {
    let mask4 = mm_set1_epi8(0x0F);
    let lookup = mm_setr_epi8(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);

    let low = mask4 & x;
    let high = mask4 & mm_srli_epi16(x, 4);

    let low_count = mm_shuffle_epi8(lookup, low);
    let high_count = mm_shuffle_epi8(lookup, high);
    let count8 = mm_add_epi8(low_count, high_count);

    let count16 = mm_add_epi8(count8, mm_slli_epi16(count8, 8));
    return mm_srli_epi16(count16, 8);
}

#[cfg(test)]
mod tests {
    use x86intrin::*;
    use sseext;

    #[test]
    fn test_mm_popcnt_epi16() {
        let m1 = mm_setr_epi16(0x0000, 0x0001, 0x0010, 0x0100, 0x1000, 0x1100, 0x0011, 0x0101);
        let m2 = mm_setr_epi16(0x1110, 0x1101, 0x1011, 0x0111, 0xFF00u16 as i16, 0x00FF, 0x0F0F, 0xFFFFu16 as i16);

        assert_eq!(sseext::mm_popcnt_epi16(m1).as_i16x8().as_array(), [0, 1, 1, 1, 1, 2, 2, 2]);
        assert_eq!(sseext::mm_popcnt_epi16(m2).as_i16x8().as_array(), [3, 3, 3, 3, 8, 8, 8, 16]);
    }
}
