mod direction;
mod events;
mod game;
mod menu;
mod position;
mod screen;
mod snake;
mod welcome;
mod world;

use direction::*;
use events::*;
use game::*;
use menu::*;
use position::*;
use screen::*;
use snake::*;
use welcome::*;
use world::*;

fn main() {
    Game::new().init();
}
