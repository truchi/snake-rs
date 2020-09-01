//! Physics utilities

mod direction;
mod moving;
mod point;
mod speed;
mod speed2d;

pub use direction::*;
pub use f64 as Distance;
pub use moving::*;
pub use point::*;
pub use speed::*;
pub use speed2d::*;
pub use std::time::Duration;
