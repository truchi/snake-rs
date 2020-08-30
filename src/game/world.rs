use super::Snake;
use crate::{
    events::KeyCode,
    geometry::{Direction, Location, Point},
};
use crossterm::terminal::{Clear, ClearType};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct World {
    bounds: Point,
    snake:  Snake,
    // eatables: Vec<Position>,
}

impl World {
    pub fn new(bounds: Point) -> Self {
        let snake = Snake::new((0, 0), Direction::Right, (20, 8));

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
        if let Some(head) = self.snake.update() {
            match head.is_inside((0, 0).into(), self.bounds) {
                Location::Inside =>
                    if !self.snake.contains(head) {
                        self.snake.step()
                    },
                _ => panic!(),
            }
        }
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
