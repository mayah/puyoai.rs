use frame;
use plain_field::PuyoPlainField;

#[test]
fn test_simulate1() {
    let mut pf = PuyoPlainField::from_str("..RRRR");
    let rensa_result = pf.simulate();

    assert_eq!(rensa_result.chains, 1);
    assert_eq!(rensa_result.score, 40);
    assert_eq!(rensa_result.frames, frame::FRAMES_VANISH_ANIMATION);
    assert!(rensa_result.quick);
}

#[test]
fn test_simulate2() {
    let mut pf = PuyoPlainField::from_str(concat!(
        "..B...",
        "..BBYB",
        "RRRRBB"));
    let rensa_result = pf.simulate();

    let expected_frames =
        frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP[1] + frame::FRAMES_GROUNDING +
        frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP[1] + frame::FRAMES_GROUNDING;

    assert_eq!(rensa_result.chains, 2);
    assert_eq!(rensa_result.score, 700);
    assert_eq!(rensa_result.frames, expected_frames);
    assert!(!rensa_result.quick);
}
