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
    body:      VecDeque<Point>,
    direction: Direction,
    speed:     Speed2D,
    last_tail: Option<Point>,
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
        let last_tail = None;

        Self {
            position,
            body,
            direction,
            speed: speed.into(),
            last_tail,
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

    pub fn grow_head(&mut self, i: u8) {
        for _ in 0..i {
            let head = self.head();
            let direction: Point = self.direction.into();
            self.body.push_front(head + direction);
        }
    }

    pub fn grow_tail(&mut self) {
        if let Some(last_tail) = self.last_tail.take() {
            self.body.push_front(last_tail);
        }
    }

    pub fn shrink(&mut self, i: u8) {
        for _ in 0..i {
            self.last_tail = self.body.pop_back();
        }
    }

    pub fn contains(&self, position: impl Into<Point>) -> bool {
        self.body.contains(&position.into())
    }
}

impl Moving for Snake {
    fn r#move(&mut self, duration: Duration) {
        let prev_position = self.position.round();
        (&mut self.position, self.speed, self.direction).r#move(duration);

        let position = self.position.round();
        if position != prev_position {
            let grow = (position - prev_position).length().round() as u8;
            self.grow_head(grow);
            self.shrink(grow);
        }
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
