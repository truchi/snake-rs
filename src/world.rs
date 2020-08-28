use crate::{Direction, KeyCode, Position, Snake};
use crossterm::terminal::{Clear, ClearType};
use std::fmt::{Display, Formatter};

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

    pub fn handle(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => self.set_direction(Direction::Up),
            KeyCode::Down => self.set_direction(Direction::Down),
            KeyCode::Left => self.set_direction(Direction::Left),
            KeyCode::Right => self.set_direction(Direction::Right),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.snake.update();
    }

    fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
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
