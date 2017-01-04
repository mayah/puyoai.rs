//! puyoai-data defines FrameRequest, FrameResponse etc.
//! They will be used for client-server communication.

extern crate puyoai_core;

pub mod game_result;
pub mod frame_request;
pub mod user_event;

pub use game_result::GameResult;
pub use user_event::UserEvent;
