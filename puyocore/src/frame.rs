// 1 seconds = 60 frames.
pub const FPS: usize = 60;

// The number of frames for preparing puyo. This frame is for animation that
// a puyo in NEXT is moving to a player field.
pub const FRAMES_PREPARING_NEXT: usize = 12;

// The number of frames for grounding animation of puyo.
pub const FRAMES_GROUNDING: usize = 20;

// The number of frames for vanishing animation of puyo.
pub const FRAMES_VANISH_ANIMATION: usize = 50;

// After this frames passed after the puyo is controllable, NEXT2 will be shown.
pub const FRAMES_NEXT2_DELAY: usize = 16;
pub const FRAMES_FREE_FALL: usize = 16;
pub const FRAMES_QUICKTURN: usize = 20;

pub const FRAMES_CONTINUOUS_TURN_PROHIBITED: usize = 3;
pub const FRAMES_CONTINUOUS_ARROW_PROHIBITED: usize = 3;

// dropping after chigiri or dropping ojama puyo.
pub const FRAMES_TO_DROP: &'static[usize] = &[
    0, 10, 16, 22, 24, 28, 32, 34, 36, 40, 42, 44, 46, 48, 50, 52
];

// Pressing DOWN, or dropping after rensa.
pub const FRAMES_TO_DROP_FAST: &'static[usize] = &[
    0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
];

//
pub const FRAMES_TO_MOVE_HORIZONTALLY: &'static[usize] = &[
    0, 4, 6, 8, 10, 12
];

pub fn grounding() -> usize {
    FRAMES_GROUNDING
}

pub fn frames_to_drop_fast(num_drop: usize) -> usize {
    FRAMES_TO_DROP_FAST[num_drop]
}

// Returns the number of animation frames when ojama is grounding
// TODO(mayah): This is not accurate.
pub fn frames_grounding_ojama(num_ojama: usize) -> usize {
    if num_ojama <= 0 {
        return 0;
    }
    if num_ojama <= 3 {
        return 8;
    }
    if num_ojama <= 18 {
        return 24;
    }
    32
}
