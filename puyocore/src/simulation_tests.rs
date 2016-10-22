use bit_field::BitField;
use frame;
use plain_field::PuyoPlainField;
use rensa::RensaResult;

fn run_plainfield_test(mut pf: PuyoPlainField, expected_result: &RensaResult) {
    let actual_result = pf.simulate();

    assert_eq!(actual_result.chains, expected_result.chains);
    assert_eq!(actual_result.score, expected_result.score);
    // Check only frames is not zero.
    if expected_result.frames > 0 {
        assert_eq!(actual_result.frames, expected_result.frames);
    }
    assert_eq!(actual_result.quick, expected_result.quick);
}

fn run_bitfield_test(mut bf: BitField, expected_result: &RensaResult) {
    let actual_result = bf.simulate();

    assert_eq!(actual_result.chains, expected_result.chains);
    assert_eq!(actual_result.score, expected_result.score);
    // Check only frames is not zero.
    if expected_result.frames > 0 {
        assert_eq!(actual_result.frames, expected_result.frames);
    }
    assert_eq!(actual_result.quick, expected_result.quick);
}

fn run_test(original: PuyoPlainField, expected_result: RensaResult) {
    run_plainfield_test(original.clone(), &expected_result);
    run_bitfield_test(BitField::from_plain_field(original), &expected_result);
}

#[test]
fn test_simulate_1rensa_quick() {
    let pf = PuyoPlainField::from_str("..RRRR");

    let expected_result = RensaResult {
        chains: 1,
        score: 40,
        frames: frame::FRAMES_VANISH_ANIMATION,
        quick: true,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case1() {
    let pf = PuyoPlainField::from_str(concat!(
        ".....Y",
        "..RRRR",
    ));

    let expected_result = RensaResult {
        chains: 1,
        score: 40,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case2() {
    let pf = PuyoPlainField::from_str(concat!(
        ".....Y",
        ".....R",
        "...RRR",
    ));

    let expected_result = RensaResult {
        chains: 1,
        score: 40,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_1rensa_nonquick_case3() {
    let pf = PuyoPlainField::from_str(concat!(
        ".....Y",
        "....YR",
        "...RRR",
    ));

    let expected_result = RensaResult {
        chains: 1,
        score: 40,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_2rensa_quick() {
    let pf = PuyoPlainField::from_str(concat!(
        "Y.....",
        "RYY...",
        "RRRY..",
    ));

    let expected_result = RensaResult {
        chains: 2,
        score: 40 + 40 * 8,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION,
        quick: true,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_2rensa_nonquick_case1() {
    let pf = PuyoPlainField::from_str(concat!(
        "YB....",
        "RYY...",
        "RRRY..",
    ));

    let expected_result = RensaResult {
        chains: 2,
        score: 40 + 40 * 8,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[2] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_2rensa_nonquick_case2() {
    let pf = PuyoPlainField::from_str(concat!(
        "..B...",
        "..BBYB",
        "RRRRBB"));

    let expected_result = RensaResult {
        chains: 2,
        score: 700,
        frames: frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING +
            frame::FRAMES_VANISH_ANIMATION + frame::FRAMES_TO_DROP_FAST[1] + frame::FRAMES_GROUNDING,
        quick: false
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate_19rensa_case1() {
    let pf = PuyoPlainField::from_str(concat!(
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
        "RBGBGG"));

    let expected_result = RensaResult {
        chains: 19,
        score: 175080,
        frames: 0,
        quick: true,
    };

    run_test(pf, expected_result);
}

#[test]
fn test_simulate() {
    run_test(PuyoPlainField::from_str(".B.GRBBGRRYRRRYYGYRBYRGRYBYRGYYBGBYRBRGBYRGRGYGYRYYGYYGRGYGRGBGYRRYBGBGBRGBGBB"),
             RensaResult::new(19, 175080, 0, true));
    run_test(PuyoPlainField::from_str("B..RYGGYGYGBGRRRBRGBRYBGRRGGYGYYGYRRYGRBRBRBBGYGRGGYRRRBGRGRYBYRBBRRYGGBRBBYRY"),
             RensaResult::new(19, 175080, 0, true));
    run_test(PuyoPlainField::from_str("...BB..B.RBB.RBRBO.RBGRB.GRGRB.GRYRB.YGYGR.YGYGR.BYBYG.BYBYGBOBOYGRRRROYBBBOBB"),
             RensaResult::new(2, 38540, 0, true));
    run_test(PuyoPlainField::from_str(".B.BB..R.RBB.GBRBO.GBGRB.YRGRB.YRYRB.YGYGR.BGYGRGRGBGRGGYBYGYGBOBYYRRROBBBBOBB"),
             RensaResult::new(3, 43260, 0, true));
    run_test(PuyoPlainField::from_str("...BB..R.RBB.GBRBOGGBGRBRYRGRBRYRYRBRYGYGRRBGYGRORGBGRGGYBYGYGBOBYYRRROBBBBOBB"),
             RensaResult::new(4, 50140, 0, true));
    run_test(PuyoPlainField::from_str("GRBBB.BGYRBBYYYRBOOGBGRBBYRGRBBYRGRBBYGYGROBGYGRGRGBGRGGYBYYYOBOBYYRRROBBBBOBB"),
             RensaResult::new(5, 68700, 0, true));
    run_test(PuyoPlainField::from_str("RRRROOOOROROROROOROOROOORORORRROOOORROROOOOORRORROOOORRROOOROOOORORRROROOOOORO"),
             RensaResult::new(4, 4840, 0, true));
    run_test(PuyoPlainField::from_str("BRBBRRBRRRBRRBRBRBRBRBRBRBRBRBBRBRBRRRBBRRBBRRBBRBRBRBBRBRBRBBRBRRRRBRBBRBBRRB"),
             RensaResult::new(9, 49950, 0, true));
    run_test(PuyoPlainField::from_str("RRRRRYBRRYOORRYOYRBYRRROBRYOYYBYBYOBRBRBBORRORRROOOOOOOOOOOOOOOOOOOOOOOOOOOOOO"),
             RensaResult::new(9, 32760, 0, true));
    run_test(PuyoPlainField::from_str("YYGBRGRYYBBBYYOYGGRGORGBRBORRGRYOYYYYYOBRGRBGRGGBBYRRYGGYBBBGRRYRYRGYRYYGRRBBB"),
             RensaResult::new(18, 155980, 0, true));
    run_test(PuyoPlainField::from_str("RRR.RRORRROROORORRROOROORORORRORORORRORORORROROROORROORROORRROROROORORORORRORR"),
             RensaResult::new(11, 47080, 0, true));
    run_test(PuyoPlainField::from_str("......RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR"),
             RensaResult::new(1, 7200, 0, true));
}
