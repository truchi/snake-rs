use super::Direction;
use std::{
    fmt::{Debug, Formatter},
    ops::Add,
};

#[derive(Debug)]
pub enum Location {
    Top,
    Bottom,
    Left,
    Right,
    Inside,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn is_inside(&self, top_left: Self, bottom_right: Self) -> Location {
        let Self { x, y } = *self;
        let Self { x: top, y: left } = top_left;
        let Self {
            x: bottom,
            y: right,
        } = bottom_right;

        return if x < top {
            Location::Top
        } else if x > bottom {
            Location::Bottom
        } else if y < left {
            Location::Left
        } else if y > right {
            Location::Right
        } else {
            Location::Inside
        };
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point::new(x, y)
    }
}

impl From<Point> for (i32, i32) {
    fn from(Point { x, y }: Point) -> Self {
        (x, y)
    }
}

impl From<Direction> for Point {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

impl<T: Into<Point>> Add<T> for Point {
    type Output = Point;

    fn add(self, position: T) -> Self::Output {
        let Point { x, y } = position.into();

        Point::new(self.x + x, self.y + y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}
