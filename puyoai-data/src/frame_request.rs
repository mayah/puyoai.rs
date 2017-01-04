use puyoai_core::field::PuyoPlainField;
use puyoai_core::kumipuyo::Kumipuyo;
use puyoai_core::kumipuyo::KumipuyoPos;

use user_event::UserEvent;
use game_result::GameResult;

pub struct PlayerFrameRequest {
    pub field: PuyoPlainField,
    pub seq: Vec<Kumipuyo>,
    pub pos: KumipuyoPos,
    pub event: UserEvent,
    pub score: u32,
    pub ojama: u32,
}

pub struct FrameRequest {
    pub frame_id: i32,
    pub game_result: GameResult,
    pub match_end: bool,
    pub player_frame_request: [PlayerFrameRequest; 2],
}
