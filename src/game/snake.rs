use crate::physics::{Direction, Point, Speed2D};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

/// The infamous `Snake`
#[derive(Debug)]
pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    speed: Speed2D,
}

impl Snake {
    pub fn new(
        position: impl Into<Point>,
        direction: impl Into<Direction>,
        speed: impl Into<Speed2D>,
    ) -> Self {
        let mut body = VecDeque::new();
        body.push_front(position.into());

        Self {
            body,
            direction: direction.into(),
            speed: speed.into(),
        }
    }

    pub fn set_direction(&mut self, direction: impl Into<Direction>) {
        let direction = direction.into();

        if self.direction.opposite() != direction {
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
                MoveTo(position.x.as_units(), position.y.as_units())
            )?;
        }

        Ok(())
    }
}
