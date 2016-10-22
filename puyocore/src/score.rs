pub const SCORE_FOR_OJAMA: usize = 70;
pub const ZENKESHI_BONUS_SCORE: usize = SCORE_FOR_OJAMA * 30;

const CHAIN_BONUS: [usize; 20] = [
      0,   0,   8,  16,  32,  64,  96, 128, 160, 192,
    224, 256, 288, 320, 352, 384, 416, 448, 480, 512,
];

const COLOR_BONUS: [usize; 6] = [
    0, 0, 3, 6, 12, 24,
];

const LONG_BONUS: [usize; 12] = [
    0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10,
];

pub fn score_for_ojama(num_ojama: usize) -> usize {
    num_ojama * SCORE_FOR_OJAMA
}

pub fn chain_bonus(nth_chain: usize) -> usize {
    debug_assert!(nth_chain <= 19, "nth_chain={}", nth_chain);
    // CHAIN_BONUS[nth_chain as usize]
    unsafe {
        *CHAIN_BONUS.get_unchecked(nth_chain)
    }
}

pub fn color_bonus(num_colors: usize) -> usize {
    debug_assert!(num_colors <= 5, "num_colors={}", num_colors);
    // COLOR_BONUS[num_colors as usize]
    unsafe {
        *COLOR_BONUS.get_unchecked(num_colors)
    }
}

pub fn long_bonus(num_puyos: usize) -> usize {
    let n = if num_puyos > 11 { 11 } else { num_puyos };
    // LONG_BONUS[n as usize]
    unsafe {
        *LONG_BONUS.get_unchecked(n)
    }
}

pub fn calculate_rensa_bonus_coef(chain_bonus_coef: usize,
                                  long_bonus_coef: usize,
                                  color_bonus_coef: usize) -> usize {
    let coef = chain_bonus_coef + long_bonus_coef + color_bonus_coef;
    if coef == 0 {
        1
    } else if coef > 999 {
        999
    } else {
        coef
    }
}

#[cfg(test)]
mod tests {
    use score::calculate_rensa_bonus_coef;

    #[test]
    fn test_rensa_bonus_coef() {
        assert_eq!(calculate_rensa_bonus_coef(0, 0, 0), 1);
        assert_eq!(calculate_rensa_bonus_coef(999, 12, 12), 999);
        assert_eq!(calculate_rensa_bonus_coef(0, 0, 2), 2);
    }
}
