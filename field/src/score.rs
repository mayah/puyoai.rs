pub const SCORE_FOR_OJAMA: i32 = 70;
pub const ZENKESHI_BONUS_SCORE: i32 = SCORE_FOR_OJAMA * 30;

const CHAIN_BONUS: [i32; 20] = [
      0,   0,   8,  16,  32,  64,  96, 128, 160, 192,
    224, 256, 288, 320, 352, 384, 416, 448, 480, 512,
];

const COLOR_BONUS: [i32; 6] = [
    0, 0, 3, 6, 12, 24,
];

const LONG_BONUS: [i32; 12] = [
    0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10,
];

pub fn score_for_ojama(num_ojama: i32) -> i32 {
    num_ojama * SCORE_FOR_OJAMA
}

pub fn chain_bonus(nth_chain: i32) -> i32 {
    debug_assert!(0 <= nth_chain && nth_chain <= 19, "nth_chain={}", nth_chain);
    // CHAIN_BONUS[nth_chain as usize]
    unsafe {
        *CHAIN_BONUS.get_unchecked(nth_chain as usize)
    }
}

pub fn color_bonus(num_colors: i32) -> i32 {
    debug_assert!(0 <= num_colors && num_colors <= 5, "num_colors={}", num_colors);
    // COLOR_BONUS[num_colors as usize]
    unsafe {
        *COLOR_BONUS.get_unchecked(num_colors as usize)
    }
}

pub fn long_bonus(num_puyos: i32) -> i32 {
    debug_assert!(0 <= num_puyos, "num_puyos={}", num_puyos);
    let n = if num_puyos > 11 { 11 } else { num_puyos };
    // LONG_BONUS[n as usize]
    unsafe {
        *LONG_BONUS.get_unchecked(n as usize)
    }
}

pub fn calculate_rensa_bonus_coef(chain_bonus_coef: i32,
                                  long_bonus_coef: i32,
                                  color_bonus_coef: i32) -> i32 {
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
