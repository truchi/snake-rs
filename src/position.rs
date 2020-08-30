use crate::Direction;
use std::{
    fmt::{Debug, Formatter},
    ops::Add,
};

#[derive(Debug)]
pub enum Inside {
    Top,
    Bottom,
    Left,
    Right,
    Inside,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn is_inside(&self, top_left: Self, bottom_right: Self) -> Inside {
        let Self { x, y } = *self;
        let Self { x: top, y: left } = top_left;
        let Self {
            x: bottom,
            y: right,
        } = bottom_right;

        return if x < top {
            Inside::Top
        } else if x > bottom {
            Inside::Bottom
        } else if y < left {
            Inside::Left
        } else if y > right {
            Inside::Right
        } else {
            Inside::Inside
        };
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Position::new(x, y)
    }
}

impl From<Position> for (i32, i32) {
    fn from(Position { x, y }: Position) -> Self {
        (x, y)
    }
}

impl From<Direction> for Position {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Position::new(0, -1),
            Direction::Down => Position::new(0, 1),
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0),
        }
    }
}

impl<T: Into<Position>> Add<T> for Position {
    type Output = Position;

    fn add(self, position: T) -> Self::Output {
        let Position { x, y } = position.into();

        Position::new(self.x + x, self.y + y)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}
