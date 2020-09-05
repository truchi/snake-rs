use super::{Food, Snake};
use crate::{
    events::KeyCode,
    physics::{Direction, Moving, PathFragment, Point, Speed2D},
};
use crossterm::terminal::{Clear, ClearType};
use std::{
    fmt::{Display, Error, Formatter},
    iter::Cycle,
    time::Duration,
    vec::IntoIter,
};

#[derive(Debug)]
pub struct World {
    bounds: Point,
    delta:  Duration,
    snake:  Snake,
    food:   Food<Cycle<IntoIter<PathFragment>>>,
}

impl World {
    pub fn new(bounds: Point, delta: Duration) -> Self {
        let snake = Snake::new(
            (0.0, 0.0),
            Direction::Right,
            Speed2D::from_per_sec((10.0, 10.0)),
        );

        let food = Food::new(
            (2.0, 2.0),
            vec![
                PathFragment::from((Duration::from_secs(3), (3.0, 0.0))),
                PathFragment::from((Duration::from_secs(3), (0.0, 3.0))),
                PathFragment::from((Duration::from_secs(3), (-3.0, 0.0))),
                PathFragment::from((Duration::from_secs(3), (0.0, -3.0))),
            ]
            .into_iter()
            .cycle(),
        );

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
        self.snake.r#move(self.delta);
        self.food.r#move(self.delta);
    }

    fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }

    pub fn render(&self) {
        println!("{}", self);
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", Clear(ClearType::All), self.snake, self.food)
    }
}
