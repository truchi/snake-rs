//! Physics utilities

mod coord_2d;
mod direction;
mod moving;
mod path;
mod path_fragment;
mod point;
mod speed;
mod speed_2d;

pub use coord_2d::*;
pub use direction::*;
pub use moving::*;
pub use path::*;
pub use path_fragment::*;
pub use point::*;
pub use speed::*;
pub use speed_2d::*;
pub use std::time::Duration;

/// A `Distance` type (`f64`).
pub type Distance = f64;
