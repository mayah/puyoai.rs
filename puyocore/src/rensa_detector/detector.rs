use field::{self, CoreField};
use column_puyo_list::ColumnPuyoList;
use color::{PuyoColor, NUM_PUYO_COLORS};
use rensa_detector::PurposeForFindingRensa;

pub fn detect_by_drop<Callback>(original_field: &CoreField,
                                prohibits: &[bool],
                                purpose: PurposeForFindingRensa,
                                max_complement_puyos: usize,
                                max_puyo_height: usize,
                                mut callback: Callback)
                                where Callback: FnMut(CoreField, &ColumnPuyoList) {
    let mut visited = [[false; NUM_PUYO_COLORS]; field::MAP_WIDTH];

    let normal_color_bits = original_field.field().normal_color_bits();
    let empty_bits = original_field.field().bits(PuyoColor::EMPTY);
    let edge_bits = (normal_color_bits & empty_bits.expand_edge()).masked_field_12();

    edge_bits.iterate_bit_position(|x, y| {
        debug_assert!(original_field.is_normal_color(x, y));
        let c = original_field.color(x, y);

        for d in &[-1isize, 0isize, 1isize] {
            let xd = (x as isize + *d) as usize;
            if prohibits[xd] || visited[xd][c as usize] || xd <= 0 || field::WIDTH < xd {
                continue;
            }
            if *d == 0 {
                if !original_field.is_empty(x, y + 1) {
                    continue;
                }
                // If the first rensa is this, any rensa won't continue.
                // This is like erasing the following X.
                // ......
                // .YXY..
                // BZZZBB
                // CAAACC
                //
                // So, we should be able to skip this.
                if purpose == PurposeForFindingRensa::ForFire && !original_field.is_connected(x, y) {
                    continue;
                }
            } else {
                if !original_field.is_empty(xd, y) {
                    continue;
                }
            }

            visited[xd][c as usize] = true;
            let mut necessary_puyos = 0;

            let mut ok = true;
            let mut cf: CoreField = (*original_field).clone();
            loop {
                if !cf.drop_puyo_on_with_max_height(xd, c, max_puyo_height) {
                    ok = false;
                    break;
                }

                necessary_puyos += 1;
                if max_complement_puyos < necessary_puyos {
                    ok = false;
                    break;
                }
                if cf.count_connected_max4_with_color(xd, cf.height(xd), c) >= 4 {
                    break;
                }
            }

            if !ok {
                continue;
            }

            let mut cpl = ColumnPuyoList::new();
            if !cpl.add_multi(xd, c, necessary_puyos) {
                continue;
            }

            callback(cf, &cpl);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use column_puyo_list::ColumnPuyoList;
    use field::CoreField;
    use rensa_detector::PurposeForFindingRensa;

    #[test]
    fn test_detect_by_drop() {
        let original = CoreField::from_str(concat!(
            ".RGYG.",
            "RGYGB.",
            "RGYGB.",
            "RGYGB.",
        ));

        let expected = CoreField::from_str(concat!(
            ".RGYG.",
            "RGYGB.",
            "RGYGB.",
            "RGYGBB",
        ));

        let mut found = false;
        {
            let callback = |actual: CoreField, cpl: &ColumnPuyoList| {
                if actual != expected {
                    return;
                }

                assert!(!found);
                found = true;

                let mut cf = original.clone();
                assert!(cf.drop_column_puyo_list(cpl));
                assert_eq!(expected, cf);
            };

            let no_prohibits = &[false; 8];
            detect_by_drop(&original, no_prohibits, PurposeForFindingRensa::ForFire, 1, 12, callback);
        }
        assert!(found);
    }
}
