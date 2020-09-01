use crate::physics::{Direction, Moving, Point, Speed2D};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

/// The infamous `Snake`
#[derive(Debug)]
pub struct Snake {
    position:  Point,
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
        let position = position.into();
        let mut body = VecDeque::new();
        body.push_front(position);

        Self {
            position,
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

    pub fn grow(&mut self, point: impl Into<Point>) {
        self.body.push_front(point.into());
    }

    pub fn shrink(&mut self) {
        self.body.pop_back();
    }

    pub fn contains(&self, position: impl Into<Point>) -> bool {
        self.body.contains(&position.into())
    }
}

impl Moving for Snake {
    fn position(&mut self) -> &mut Point {
        &mut self.position
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn speed(&self) -> Speed2D {
        self.speed
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for point in &self.body {
            write!(
                f,
                "{}🐍", // 🍭🐍👅🦀
                MoveTo(point.x as u16, point.y as u16)
            )?;
        }

        Ok(())
    }
}
