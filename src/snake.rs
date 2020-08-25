use crate::{Direction, Position};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Bang {
    Up,
    Down,
    Left,
    Right,
    Snake,
}

#[derive(Debug)]
pub struct Snake {
    body:      VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    pub fn new(position: impl Into<Position>, direction: Direction) -> Self {
        let mut body = VecDeque::new();
        body.push_front(position.into());

        Self { body, direction }
    }

    pub fn head(&self) -> Position {
        *self.body.front().expect("Snake body is empty")
    }

    pub fn slither(&mut self, max: Position) -> Result<(), Bang> {
        let (x, y) = self.head().into();

        match (x, y, self.direction) {
            (_, y, Direction::Up) if y == 0 => Err(Bang::Up),
            (_, y, Direction::Down) if y == max.y() => Err(Bang::Down),
            (x, _, Direction::Left) if x == 0 => Err(Bang::Left),
            (x, _, Direction::Right) if x == max.x() => Err(Bang::Right),
            (x, y, direction) => {
                let new_head = Position::new(x, y) + direction;

                if self.body.contains(&new_head) {
                    Err(Bang::Snake)
                } else {
                    self.body.pop_back();
                    self.body.push_front(new_head);
                    Ok(())
                }
            }
        }
    }

    pub fn grow(&mut self) {
        ()
    }
}
