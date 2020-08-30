use super::Direction;
use std::{
    fmt::{Debug, Formatter},
    ops::Add,
};

#[derive(Eq, PartialEq, Debug)]
pub enum Location {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Inside,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_inside(&self, top_left: Self, bottom_right: Self) -> Location {
        let Self { x, y } = *self;
        let Self { x: top, y: left } = top_left;
        let Self {
            x: bottom,
            y: right,
        } = bottom_right;

        return if x < top {
            if y < left {
                Location::TopLeft
            } else if y > right {
                Location::TopRight
            } else {
                Location::Top
            }
        } else if x > bottom {
            if y < left {
                Location::BottomLeft
            } else if y > right {
                Location::BottomRight
            } else {
                Location::Bottom
            }
        } else if y < left {
            Location::Left
        } else if y > right {
            Location::Right
        } else {
            Location::Inside
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn is_inside() {
        let top_left_corner = Point::new(0, 0);
        let bottom_right_corner = Point::new(10, 10);

        let top = Point::new(-1, 5);
        let bottom = Point::new(20, 5);
        let left = Point::new(5, -1);
        let right = Point::new(5, 20);
        let top_left = Point::new(-1, -1);
        let top_right = Point::new(-1, 20);
        let bottom_left = Point::new(20, -1);
        let bottom_right = Point::new(20, 20);

        for (point, location) in &[
            (top, Location::Top),
            (bottom, Location::Bottom),
            (left, Location::Left),
            (right, Location::Right),
            (top_left, Location::TopLeft),
            (top_right, Location::TopRight),
            (bottom_left, Location::BottomLeft),
            (bottom_right, Location::BottomRight),
        ] {
            assert_eq!(
                point.is_inside(top_left_corner, bottom_right_corner),
                *location
            );
        }
    }

    #[test]
    fn from_direction() {
        assert_eq!(Point::from(Direction::Up), Point::new(0, -1));
        assert_eq!(Point::from(Direction::Down), Point::new(0, 1));
        assert_eq!(Point::from(Direction::Left), Point::new(-1, 0));
        assert_eq!(Point::from(Direction::Right), Point::new(1, 0));
    }

    #[test]
    fn add() {
        assert_eq!(Point::new(1, 2) + Point::new(3, 4), Point::new(4, 6));
        assert_eq!(Point::new(9, 8) + Direction::Left, Point::new(8, 8));
    }
}
