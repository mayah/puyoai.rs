use field::{FieldHeight, FieldIsEmpty};
use decision::Decision;

pub struct PuyoController {
}

impl PuyoController {
    pub fn new() -> PuyoController {
        PuyoController {
        }
    }

    pub fn is_reachable<F: FieldHeight + FieldIsEmpty>(&self, field: &F, decision: &Decision) -> bool {
        debug_assert!(decision.is_valid());

        const CHECKER: &'static [&'static [usize]] = &[
            &[3, 2, 1, 0],
            &[3, 2, 0],
            &[3, 0],
            &[3, 4, 0],
            &[3, 4, 5, 0],
            &[3, 4, 5, 6, 0],
        ];

        let mut checker_idx = decision.axis_x() - 1;
        if decision.rot() == 1 && 3 <= decision.axis_x() {
            checker_idx += 1;
        } else if decision.rot() == 3 && decision.axis_x() <= 3 {
            checker_idx -= 1;
        }

        let mut y_might_be_13 = field.height(2) >= 12 && field.height(4) >= 12;
        let mut i = 1;

        while CHECKER[checker_idx][i] != 0 {
            let x = CHECKER[checker_idx][i];
            if field.height(x) <= 11 {
                y_might_be_13 = false;
                i += 1;
                continue;
            }

            if field.height(x) == 12 {
                if y_might_be_13 {
                    i += 1;
                    continue;
                }

                if field.height(CHECKER[checker_idx][i - 1]) == 11 {
                    y_might_be_13 = true;
                    i += 1;
                    continue;
                }
                if i >= 2 && field.height(CHECKER[checker_idx][i - 2]) == 12 {
                    y_might_be_13 = true;
                    i += 1;
                    continue;
                }
            }

            return false;
        }

        if decision.rot() == 2 && field.height(decision.axis_x()) >= 12 {
            return false;
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use field::{FieldWithHeight, PuyoPlainField};
    use decision::Decision;

    #[test]
    fn test_is_reachable_empty_field() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::new());

        for d in Decision::all_valid_decisions() {
            assert!(pc.is_reachable(&f, d));
        }
    }

    #[test]
    fn test_is_reachable_upper_1() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            "......", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        for d in Decision::all_valid_decisions() {
            assert!(pc.is_reachable(&f, d));
        }
    }

    #[test]
    fn test_is_reachable_upper_2() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            "......", // 12
            "......",
            ".....O",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        for d in Decision::all_valid_decisions() {
            assert!(pc.is_reachable(&f, d));
        }
    }

    #[test]
    fn test_is_reachable_upper_3() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            "O....O", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let unreachables = [
            Decision::new(1, 2),
            Decision::new(6, 2)
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), !unreachables.contains(d));
        }
    }

    #[test]
    fn test_is_reachable_upper_4() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            ".O..O.", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let unreachables = [
            Decision::new(2, 2),
            Decision::new(5, 2)
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), !unreachables.contains(d));
        }
    }

    #[test]
    fn test_is_reachable_upper_5() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            "...O..", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let unreachables = &[
            Decision::new(4, 2),
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), !unreachables.contains(d));
        }
    }

    #[test]
    fn test_is_reachable_upper_6() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            ".O.O..", // 12
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let unreachables = [
            Decision::new(2, 2),
            Decision::new(4, 2),
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), !unreachables.contains(d));
        }
    }

    #[test]
    fn test_is_reachable_upper_7() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            ".O.O..", // 12
            ".O.O..",
            ".O.O..",
            ".O.O..",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let unreachables = [
            Decision::new(2, 2),
            Decision::new(4, 2),
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), !unreachables.contains(d));
        }
    }

    #[test]
    fn test_is_reachable_upper_8() {
        let pc = PuyoController::new();
        let f = FieldWithHeight::from_field(PuyoPlainField::from_str(concat!(
            "O...OO", // 13
            "OO.OOO", // 12
            "OO.OOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 8
            "OOOOOO",
            "OOOOOO",
            "OOOOOO",
            "OOOOOO", // 4
            "OOOOOO",
            "OOOOOO",
            "OOOOOO")));

        let reachables = [
            Decision::new(2, 0),
            Decision::new(2, 1),
            Decision::new(3, 0),
            Decision::new(3, 1),
            Decision::new(3, 2),
            Decision::new(3, 3),
            Decision::new(4, 0),
            Decision::new(4, 3),
        ];

        for d in Decision::all_valid_decisions() {
            assert_eq!(pc.is_reachable(&f, d), reachables.contains(d), "d={:?}", d);
        }
    }
}
