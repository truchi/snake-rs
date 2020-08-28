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

use std::{thread::sleep, time::Duration};

fn main() {
    let mut game = Game::new();
    game.init();

    // let screen = Screen::new();
    // let mut world = World::new(screen.bounds());
    // world.render();
    // event_loop(&mut world);

    // sleep(Duration::from_millis(1000));
}

fn event_loop(world: &mut World) {
    let FPS = 10;
    loop {
        let events = poll();
        // world.update(events);
        world.render();
        sleep(Duration::from_millis(200));
    }
}
