use field::PuyoPlainField;
use frame;
use rensa_result::RensaResult;

#[cfg(all(target_feature = "avx2", target_feature = "bmi2"))]
use field::BitField;

fn run_puyoplainfield_test(mut pf: PuyoPlainField, expected_result: &RensaResult) {
    let actual_result = pf.simulate();

    assert_eq!(actual_result.chain, expected_result.chain);
    assert_eq!(actual_result.score, expected_result.score);
    // Check only frames is not zero.
    if expected_result.frame > 0 {
        assert_eq!(actual_result.frame, expected_result.frame);
    }
    assert_eq!(actual_result.quick, expected_result.quick);
}

#[cfg(all(target_feature = "avx2", target_feature = "bmi2"))]
fn run_bitfield_test(mut bf: BitField, expected_result: &RensaResult) {
    let actual_result = bf.simulate();

    assert_eq!(actual_result.chain, expected_result.chain);
    assert_eq!(actual_result.score, expected_result.score);
    // Check only frames is not zero.
    if expected_result.frame > 0 {
        assert_eq!(actual_result.frame, expected_result.frame);
    }
    assert_eq!(actual_result.quick, expected_result.quick);
}

#[cfg(all(target_feature = "avx2", target_feature = "bmi2"))]
fn run_test(src: &str, expected_result: RensaResult) {
    run_puyoplainfield_test(PuyoPlainField::from_str(src), &expected_result);
    run_bitfield_test(BitField::from_str(src), &expected_result);
}

#[cfg(not(all(target_feature = "avx2", target_feature = "bmi2")))]
fn run_test(src: &str, expected_result: RensaResult) {
    run_puyoplainfield_test(PuyoPlainField::from_str(src), &expected_result);
}

#[test]
fn test_simulate_1rensa_quick() {
    let src = "..RRRR";

    let expected_result = RensaResult {
        chain: 1,
        score: 40,
        frame: frame::FRAMES_VANISH_ANIMATION,
        quick: true,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case1() {
    let src = concat!(
        ".....Y",
        "..RRRR");

    let expected_result = RensaResult {
        chain: 1,
        score: 40,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case2() {
    let src = concat!(
        ".....Y",
        ".....R",
        "...RRR");

    let expected_result = RensaResult {
        chain: 1,
        score: 40,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case3() {
    let src = concat!(
        ".....Y",
        "....YR",
        "...RRR");

    let expected_result = RensaResult {
        chain: 1,
        score: 40,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_2rensa_quick() {
    let src = concat!(
        "Y.....",
        "RYY...",
        "RRRY..");

    let expected_result = RensaResult {
        chain: 2,
        score: 40 + 40 * 8,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION,
        quick: true,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_2rensa_nonquick_case1() {
    let src = concat!(
        "YB....",
        "RYY...",
        "RRRY..");

    let expected_result = RensaResult {
        chain: 2,
        score: 40 + 40 * 8,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_2rensa_nonquick_case2() {
    let src = concat!(
        "..B...",
        "..BBYB",
        "RRRRBB");

    let expected_result = RensaResult {
        chain: 2,
        score: 700,
        frame: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate_19rensa_case1() {
    let src = concat!(
        ".G.BRG",
        "GBRRYR",
        "RRYYBY",
        "RGYRBR",
        "YGYRBY",
        "YGBGYR",
        "GRBGYR",
        "BRBYBY",
        "RYYBYY",
        "BRBYBR",
        "BGBYRR",
        "YGBGBG",
        "RBGBGG");

    let expected_result = RensaResult {
        chain: 19,
        score: 175080,
        frame: 0,
        quick: true,
    };

    run_test(src, expected_result);
}

#[test]
fn test_simulate() {
    run_test(".B.GRBBGRRYRRRYYGYRBYRGRYBYRGYYBGBYRBRGBYRGRGYGYRYYGYYGRGYGRGBGYRRYBGBGBRGBGBB",
             RensaResult::new(19, 175080, 0, true));
    run_test("B..RYGGYGYGBGRRRBRGBRYBGRRGGYGYYGYRRYGRBRBRBBGYGRGGYRRRBGRGRYBYRBBRRYGGBRBBYRY",
             RensaResult::new(19, 175080, 0, true));
    run_test("...BB..B.RBB.RBRBO.RBGRB.GRGRB.GRYRB.YGYGR.YGYGR.BYBYG.BYBYGBOBOYGRRRROYBBBOBB",
             RensaResult::new(2, 38540, 0, true));
    run_test(".B.BB..R.RBB.GBRBO.GBGRB.YRGRB.YRYRB.YGYGR.BGYGRGRGBGRGGYBYGYGBOBYYRRROBBBBOBB",
             RensaResult::new(3, 43260, 0, true));
    run_test("...BB..R.RBB.GBRBOGGBGRBRYRGRBRYRYRBRYGYGRRBGYGRORGBGRGGYBYGYGBOBYYRRROBBBBOBB",
             RensaResult::new(4, 50140, 0, true));
    run_test("GRBBB.BGYRBBYYYRBOOGBGRBBYRGRBBYRGRBBYGYGROBGYGRGRGBGRGGYBYYYOBOBYYRRROBBBBOBB",
             RensaResult::new(5, 68700, 0, true));
    run_test("RRRROOOOROROROROOROOROOORORORRROOOORROROOOOORRORROOOORRROOOROOOORORRROROOOOORO",
             RensaResult::new(4, 4840, 0, true));
    run_test("BRBBRRBRRRBRRBRBRBRBRBRBRBRBRBBRBRBRRRBBRRBBRRBBRBRBRBBRBRBRBBRBRRRRBRBBRBBRRB",
             RensaResult::new(9, 49950, 0, true));
    run_test("RRRRRYBRRYOORRYOYRBYRRROBRYOYYBYBYOBRBRBBORRORRROOOOOOOOOOOOOOOOOOOOOOOOOOOOOO",
             RensaResult::new(9, 32760, 0, true));
    run_test("YYGBRGRYYBBBYYOYGGRGORGBRBORRGRYOYYYYYOBRGRBGRGGBBYRRYGGYBBBGRRYRYRGYRYYGRRBBB",
             RensaResult::new(18, 155980, 0, true));
    run_test("RRR.RRORRROROORORRROOROORORORRORORORRORORORROROROORROORROORRROROROORORORORRORR",
             RensaResult::new(11, 47080, 0, true));
    run_test("......RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
             RensaResult::new(1, 7200, 0, true));
}
