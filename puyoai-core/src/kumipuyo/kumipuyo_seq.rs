use color::{Color, PuyoColor};
use kumipuyo::Kumipuyo;
use rand::{thread_rng, Rng};
use std::vec::Vec;

pub fn generate_random_puyocolor_sequence(size: usize) -> Vec<Kumipuyo> {
    let mut rng = thread_rng();
    let mut vs = Vec::new();
    for _ in 0 .. size {
        let axis = rng.choose(PuyoColor::all_normal_colors()).unwrap();
        let child = rng.choose(PuyoColor::all_normal_colors()).unwrap();
        vs.push(Kumipuyo::new(*axis, *child))
    }

    vs
}

pub fn generate_ac_puyo2_sequence() -> Vec<Kumipuyo> {
    let mut vs: Vec<PuyoColor> = Vec::new();
    for c in &[PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN] {
        for _ in 0 .. 64 {
            vs.push(*c)
        }
    }

    let mut rng = thread_rng();
    rng.shuffle(&mut vs[0..64 * 3]);
    rng.shuffle(&mut vs[6..64 * 4]);

    let mut ks: Vec<Kumipuyo> = Vec::new();
    for i in 0 .. 128 {
        let axis = vs[2 * i];
        let child = vs[2 * i + 1];
        ks.push(Kumipuyo::new(axis, child));
    }

    ks
}

#[cfg(test)]
mod tests {
    use color::{Color, PuyoColor};

    #[test]
    fn test_generate_random() {
        let seq = super::generate_random_puyocolor_sequence(20);

        assert_eq!(seq.len(), 20);
        for c in seq {
            assert!(c.axis().is_normal_color());
            assert!(c.child().is_normal_color());
        }
    }

    #[test]
    fn test_generate_ac_puyo2() {
        let seq = super::generate_ac_puyo2_sequence();
        assert_eq!(seq.len(), 128);

        for i in 0 .. 128 {
            let ref k = seq[i];

            assert!(k.axis().is_normal_color());
            assert!(k.child().is_normal_color());

            // The first 3 hand does not contain GREEN.
            if i <= 2 {
                assert!(k.axis() != PuyoColor::GREEN);
                assert!(k.child() != PuyoColor::GREEN);
            }
        }
    }
}
