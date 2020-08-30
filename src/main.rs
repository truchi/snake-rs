mod consts;
mod direction;
mod events;
mod game;
mod hv_stepper;
mod menu;
mod position;
mod screen;
mod snake;
mod stepper;
mod welcome;
mod world;

use consts::*;
use direction::*;
use events::*;
use game::*;
use hv_stepper::*;
use menu::*;
use position::*;
use screen::*;
use snake::*;
use stepper::*;
use welcome::*;
use world::*;

fn main() {
    // fn print_range(r: std::ops::RangeInclusive<u32>) {
    // for fps in r.clone() {
    // for cps in r.clone() {
    // print_one(cps, fps);
    // }
    // }
    // }

    Game::new().init();
}
