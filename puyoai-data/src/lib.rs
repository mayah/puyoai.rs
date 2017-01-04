//! puyoai-data defines FrameRequest, FrameResponse etc.
//! They will be used for client-server communication.

pub mod game_result;
pub mod user_event;

pub use game_result::GameResult;
pub use user_event::UserEvent;
