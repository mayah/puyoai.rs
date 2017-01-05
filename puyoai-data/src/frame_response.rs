use puyoai_core::decision::Decision;

pub struct FrameResponse {
    pub frame_id: i32,
    pub decision: Decision,
    pub message: String,
}
