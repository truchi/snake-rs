//! Physics utilities

mod direction;
// mod moving;
mod coord_2d;
mod speed;

pub use direction::*;
// pub use moving::*;
pub use coord_2d::*;
pub use speed::*;
pub use std::time::Duration;

/// A `Distance` type (`f64`).
pub type Distance = f64;

/// A `Point` type (`Coord2D<Distance>`).
pub type Point = Coord2D<Distance>;

/// A `Speed2D` type (`Coord2D<Speed<Distance>>`).
pub type Speed2D = Coord2D<Speed<Distance>>;
