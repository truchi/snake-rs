use crate::{Direction, Event, Position, Snake};
use crossterm::terminal::{Clear, ClearType};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum State {
    Playing,
    Paused,
    Resized,
}

#[derive(Debug)]
pub struct World {
    bounds: Position,
    snake:  Snake,
    // eatables: Vec<Position>,
}

impl World {
    pub fn new(bounds: Position) -> Self {
        let snake = Snake::new((0, 0), Direction::Right);

        Self { bounds, snake }
    }

    pub fn update(&mut self, events: Vec<Event>) -> State {
        State::Playing
    }

    fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }

    fn r#move(&mut self) {
        // let moved = self.snake.slither(self.screen.bounds());
        // match moved {
        // Ok(_) => (),
        // Err(err) => {
        // println!("BANG!!! {:#?}", err);
        // panic!();
        // }
        // }
        // println!("{:#?}", self.snake);

        ()
    }

    pub fn render(&self) {
        println!("{}", self);
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", Clear(ClearType::All), self.snake)
    }
}
