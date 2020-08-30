use crate::{
    geometry::{Direction, Point},
    physics::HVStepper,
};
use crossterm::cursor::MoveTo;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Snake {
    body:      VecDeque<Point>,
    direction: Direction,
    stepper:   HVStepper,
    speed:     (u64, u64),
}

impl Snake {
    pub fn new(position: impl Into<Point>, direction: Direction, speed: (u64, u64)) -> Self {
        let mut body = VecDeque::new();
        body.push_front(position.into());
        let stepper = HVStepper::new(speed.0, speed.1);

        Self {
            body,
            direction,
            stepper,
            speed,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction
        }
    }

    pub fn update(&mut self) -> Option<Point> {
        let mut position = None;

        if self.stepper.step(self.direction) {
            position = Some(self.next_head());
        }

        position
    }

    pub fn contains(&self, position: Point) -> bool {
        self.body.contains(&position)
    }

    pub fn step(&mut self) {
        self.grow();
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_front(self.next_head());
    }

    fn head(&self) -> Point {
        *self.body.front().expect("Snake body is empty")
    }

    fn next_head(&self) -> Point {
        self.head() + self.direction
    }
}

impl Display for Snake {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for position in &self.body {
            write!(
                f,
                "{}🐍", // 🍭🐍👅🦀
                MoveTo(position.x as u16, position.y as u16)
            )?;
        }

        Ok(())
    }
}
