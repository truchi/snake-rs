use crate::{Direction, Position};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Snake {
    body:      VecDeque<Position>,
    direction: Direction,
    speed:     (u64, u64),
    frames:    u64,
}

impl Snake {
    pub fn new(position: impl Into<Position>, direction: Direction, speed: (u64, u64)) -> Self {
        let mut body = VecDeque::new();
        body.push_front(position.into());
        let frames = 0;

        Self {
            body,
            direction,
            speed,
            frames,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction
        }
    }

    pub fn step(&mut self) {
        self.grow();
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_front(self.next_head());
    }

    pub fn update(&mut self) -> Option<Position> {
        let mut position = None;
        let speed;

        if self.direction.is_horizontal() {
            speed = self.speed.0;
        } else {
            speed = self.speed.1;
        }

        if self.frames % speed == 0 {
            position = Some(self.next_head());
            self.frames = 0;
        }

        self.frames += 1;

        position
    }

    fn head(&self) -> Position {
        *self.body.front().expect("Snake body is empty")
    }

    fn next_head(&self) -> Position {
        self.head() + self.direction
    }
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
