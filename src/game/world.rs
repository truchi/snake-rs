use super::{Food, Snake};
use crate::{
    events::KeyCode,
    physics::{Direction, Moving, Point},
};
use crossterm::terminal::{Clear, ClearType};
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};

#[derive(Debug)]
pub struct World {
    bounds: Point,
    delta:  Duration,
    snake:  Snake,
    food:   Food,
}

impl World {
    pub fn new(bounds: Point, delta: Duration) -> Self {
        let snake = Snake::new((0.0, 0.0), Direction::Right, (20.0, 8.0));
        let food = Food::new((10, 10));

        Self {
            bounds,
            delta,
            snake,
            food,
        }
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
        if let Some(point) = self.snake.r#move(self.delta) {
            self.snake.grow(point);
            self.snake.shrink();
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
