pub mod consts;
pub mod events;
pub mod game;
pub mod geometry;
pub mod physics;
pub mod screen;

fn main() {
    game::Game::new().init();
}
