use puyoai_core::kumipuyo::Kumipuyo;

pub struct FieldRealtime {
    player_id: usize,
    kumipuyo_seq: Vec<Kumipuyo>,
}

impl FieldRealtime {
    pub fn new(player_id: usize, seq: &[Kumipuyo]) -> FieldRealtime {
        FieldRealtime {
            player_id: player_id,
            kumipuyo_seq: seq.to_vec(),
        }
    }
}
