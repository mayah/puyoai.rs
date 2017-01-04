use field::CoreField;
use kumipuyo::Kumipuyo;
use decision::Decision;
use color::PuyoColor;

const URL_PREFIX: &'static str = "http://www.puyop.com/s/";

// 64 characters
const ENCODER: &'static[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
    '[', ']',
];

fn tsumo_color_id(c: PuyoColor) -> usize {
    match c {
        PuyoColor::RED => 0,
        PuyoColor::GREEN => 1,
        PuyoColor::BLUE => 2,
        PuyoColor::YELLOW => 3,
        // PURPLE => 4,
        _ => {
            unreachable!()
        }
    }
}

fn field_color_id(c: PuyoColor) -> usize {
    match c {
        PuyoColor::EMPTY => 0,
        PuyoColor::RED => 1,
        PuyoColor::GREEN => 2,
        PuyoColor::BLUE => 3,
        PuyoColor::YELLOW => 4,
        // PURPLE -> 5,
        PuyoColor::OJAMA => 6,
        _ => {
            unreachable!()
        }
    }
}

fn encode_control(seq: &[Kumipuyo], decisions: &[Decision]) -> String {
    let mut ss = String::new();
    for i in 0..decisions.len() {
        let kp = &seq[i];
        let mut d = tsumo_color_id(kp.axis()) * 5 + tsumo_color_id(kp.child());
        let h = (decisions[i].axis_x() << 2) + decisions[i].rot();
        d |= h << 7;
        ss.push(ENCODER[d & 0x3F] as char);
        ss.push(ENCODER[(d >> 6) & 0x3F] as char);
    }
    ss
}

fn encode_field(field: &CoreField) -> String {
    if field.is_all_cleared() {
        return "".to_owned();
    }

    let mut ss = String::new();
    let mut start = false;

    for y in (1..14).rev() {
        for px in &[1, 3, 5] {
            let x = *px;
            if !start && field.is_empty(x, y) && field.is_empty(x + 1, y) {
                continue;
            }

            let mut d = 0usize;
            d += field_color_id(field.color(x, y)) * 8;
            d += field_color_id(field.color(x + 1, y));
            assert!(d < 64);
            start = true;
            ss.push(ENCODER[d] as char);
        }
    }

    ss
}

pub fn make_puyop_url(field: &CoreField, seq: &[Kumipuyo], decisions: &[Decision]) -> String {
    if seq.is_empty() && decisions.is_empty() {
        format!("{}{}", URL_PREFIX, encode_field(field))
    } else {
        format!("{}{}_{}", URL_PREFIX, encode_field(field), encode_control(seq, decisions))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use field::CoreField;

    #[test]
    fn test_make_puyop_url() {
        let cf = CoreField::from_str(concat!(
            ".....Y",
            ".G..YY",
            "RGRRBB",
            "RRGRGB",
        ));

        assert_eq!("http://www.puyop.com/s/420Aa9r9hj",
                   make_puyop_url(&cf, &[], &[]));
    }
}
