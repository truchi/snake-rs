mod direction;
mod position;
mod screen;
mod snake;
mod world;

use direction::*;
use position::*;
use screen::*;
use snake::*;
use world::*;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;

fn main() {
    let mut world = World::new();
    println!("Hello, {:#?}", world);
    for _ in 0..100 {
        world.r#move();
    }
    // event_loop();
}

fn event_loop() {
    loop {
        if poll(Duration::from_secs(0)).unwrap() {
            let event = read().unwrap();
            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Up => {
                        println!("Up");
                    }
                    KeyCode::Down => {
                        println!("Down");
                    }
                    KeyCode::Left => {
                        println!("Left");
                    }
                    KeyCode::Right => {
                        println!("Right");
                    }
                    _ => {}
                }
            } else if let Event::Resize(..) = event {
                panic!("Resize");
            }
            break;
        }
    }
}
