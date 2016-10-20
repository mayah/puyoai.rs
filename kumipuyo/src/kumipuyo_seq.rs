use color::PuyoColor;
use kumipuyo::Kumipuyo;
use rand::{thread_rng, Rng};
use std::vec::Vec;

pub struct KumipuyoSeq{
    seq: Vec<Kumipuyo>,
}

impl KumipuyoSeq {
    pub fn new(seq: Vec<Kumipuyo>) -> KumipuyoSeq {
        KumipuyoSeq {
            seq: seq
        }
    }

    pub fn empty() -> KumipuyoSeq {
        KumipuyoSeq::new(Vec::new())
    }

    pub fn generate_random(size: usize) -> KumipuyoSeq {
        let normal_colors = [PuyoColor::RED, PuyoColor::BLUE, PuyoColor::YELLOW, PuyoColor::GREEN];

        let mut rng = thread_rng();
        let mut vs = Vec::new();
        for _ in 0 .. size {
            let axis = rng.choose(&normal_colors).unwrap();
            let child = rng.choose(&normal_colors).unwrap();
            vs.push(Kumipuyo::new(*axis, *child))
        }

        KumipuyoSeq::new(vs)
    }

    pub fn generate_ac_puyo2() -> KumipuyoSeq {
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

        KumipuyoSeq::new(ks)
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn as_slice(&self) -> &[Kumipuyo] {
        self.seq.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::KumipuyoSeq;
    use color::{Color, PuyoColor};

    #[test]
    fn test_generate_random() {
        let seq = KumipuyoSeq::generate_random(20);

        assert_eq!(seq.len(), 20);
        for c in seq.seq {
            assert!(c.axis().is_normal_color());
            assert!(c.child().is_normal_color());
        }
    }

    #[test]
    fn test_generate_ac_puyo2() {
        let seq = KumipuyoSeq::generate_ac_puyo2();
        assert_eq!(seq.len(), 128);

        for i in 0 .. 128 {
            let ref k = seq.seq[i];

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
