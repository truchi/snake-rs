use crate::physics::{Coord2D, Direction, Duration, Moving, Point, Speed2D};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Error, Formatter},
};

/// The infamous `Snake`
#[derive(Debug)]
pub struct Snake {
    position:  Point,
    pub body:  VecDeque<Point>,
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
        body.push_front(position.round());

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

    pub fn head(&self) -> Point {
        *self.body.get(0).expect("Snake have no body")
    }

    pub fn grow(&mut self, point: impl Into<Point>) {
        self.body.push_front(point.into().round());
    }

    pub fn shrink(&mut self) {
        self.body.pop_back();
    }

    pub fn contains(&self, position: impl Into<Point>) -> bool {
        self.body.contains(&position.into())
    }
}

impl Moving for Snake {
    fn r#move(&mut self, duration: Duration) {
        let duration_2d: Coord2D<Duration> = duration.into();
        let direction_2d: Point = self.direction.into();
        let new_position = self.position + (self.speed * duration_2d) * direction_2d;

        if self.position.round() != new_position.round() {
            self.grow(new_position);
        }

        self.position = new_position;
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for point in &self.body {
            write!(f, "{}🐍", MoveTo(point.x as u16 * 2, point.y as u16))?;
        }

        Ok(())
    }
}
