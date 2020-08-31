//! # Snake
//!
//! The **best** Snake

pub mod consts;
pub mod events;
pub mod game;
pub mod physics;
pub mod screen;

fn main() {
    game::Game::new().init();
}
