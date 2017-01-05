//! puyoai-data defines FrameRequest, FrameResponse etc.
//! They will be used for client-server communication.

extern crate puyoai_core;

pub mod game_result;
pub mod frame_request;
pub mod frame_response;
pub mod user_event;

pub use game_result::GameResult;
pub use frame_request::FrameRequest;
pub use frame_response::FrameResponse;
pub use user_event::UserEvent;
