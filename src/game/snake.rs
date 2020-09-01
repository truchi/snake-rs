use crate::physics::{Direction, Point, Speed2D};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

/// The infamous `Snake`
#[derive(Debug)]
pub struct Snake {
    body:      VecDeque<Point>,
    direction: Direction,
    speed:     Speed2D,
}

impl Snake {
    pub fn new(
        position: impl Into<Point>,
        direction: Direction,
        speed: impl Into<Speed2D>,
    ) -> Self {
        let mut body = VecDeque::new();
        body.push_front(position.into());

        Self {
            body,
            direction,
            speed: speed.into(),
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction != -direction {
            self.direction = direction
        }
    }

    pub fn update(&mut self) -> Option<Point> {
        let position = None;

        position
    }

    pub fn contains(&self, position: impl Into<Point>) -> bool {
        self.body.contains(&position.into())
    }

    // pub fn step(&mut self) {
    // self.grow();
    // self.body.pop_back();
    // }

    // fn head(&self) -> Point {
    // *self.body.front().expect("Snake body is empty")
    // }

    // pub fn grow(&mut self) {
    // self.body.push_front(self.next_head());
    // }

    // fn next_head(&self) -> Point {
    // self.head() + self.direction
    // }
}

impl Display for Snake {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for position in &self.body {
            write!(
                f,
                "{}🐍", // 🍭🐍👅🦀
                MoveTo(position.x.as_units() as u16, position.y.as_units() as u16)
            )?;
        }

        Ok(())
    }
}
