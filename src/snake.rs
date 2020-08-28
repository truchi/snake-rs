use crate::{Direction, Position};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

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

    pub fn next_head(&self) -> Position {
        self.head() + self.direction
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction
        }
    }

    pub fn r#move(&mut self) {
        self.body.pop_back();
        self.grow();
    }

    pub fn grow(&mut self) {
        self.body.push_front(self.next_head());
    }

    // pub fn slither(&mut self, max: Position) -> Result<(), Bang> {
    // let (x, y) = self.head().into();
    //
    // match (x, y, self.direction) {
    // (_, y, Direction::Up) if y == 0 => Err(Bang::Up),
    // (_, y, Direction::Down) if y == max.y() => Err(Bang::Down),
    // (x, _, Direction::Left) if x == 0 => Err(Bang::Left),
    // (x, _, Direction::Right) if x == max.x() => Err(Bang::Right),
    // (x, y, direction) => {
    // let new_head = Position::new(x, y) + direction;
    //
    // if self.body.contains(&new_head) {
    // Err(Bang::Snake)
    // } else {
    // self.body.pop_back();
    // self.body.push_front(new_head);
    // Ok(())
    // }
    // }
    // }
    // }
}

impl Display for Snake {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for position in &self.body {
            write!(
                f,
                "{}🐍", // 🍭🐍👅🦀
                MoveTo(position.x() as u16, position.y() as u16)
            )?;
        }

        Ok(())
    }
}
