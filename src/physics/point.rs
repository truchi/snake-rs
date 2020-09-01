use super::{Direction, Distance};
use std::{
    fmt::{Debug, Error, Formatter},
    ops::{Add, AddAssign, Rem},
};

/// A `Point` type to represent a position in cartesian space.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Point {
    /// The X axis coordinate.
    pub x: Distance,
    /// The Y axis coordinate.
    pub y: Distance,
}

impl Point {
    /// Creates a new `Point` from its `x`/`y` coordinates.
    pub fn new(x: impl Into<Distance>, y: impl Into<Distance>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    /// Returns a new `Point` only with the whole part of its coordinates,
    /// discarding the fractional parts.
    pub fn trunc(&self) -> Self {
        Self::new(self.x.trunc(), self.y.trunc())
    }

    /// Projects the `Point` along the specified `direction`.
    pub fn project(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(0, -self.y),
            Direction::Down => Self::new(0, self.y),
            Direction::Left => Self::new(-self.x, 0),
            Direction::Right => Self::new(self.x, 0),
        }
    }

    /// Adds two `Point`s together.
    pub fn add(&self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();

        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Into<Distance>, U: Into<Distance>> From<(T, U)> for Point {
    fn from((x, y): (T, U)) -> Self {
        Self::new(x, y)
    }
}

impl<T: From<Distance>, U: From<Distance>> From<Point> for (T, U) {
    fn from(point: Point) -> Self {
        (point.x.into(), point.y.into())
    }
}

/// Calls `Point::project`
impl Rem<Direction> for Point {
    type Output = Self;

    fn rem(self, rhs: Direction) -> Self {
        self.project(rhs)
    }
}

impl<T: Into<Self>> Add<T> for Point {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Self::add(&self, rhs)
    }
}

impl<T: Into<Self>> AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        *self = self.add(rhs);
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(Point::new(1.23, 4.56), Point { x: 1.23, y: 4.56 });
    }

    #[test]
    fn trunc() {
        assert_eq!(Point::new(7.777, 8.888).trunc(), Point::new(7.0, 8.0));
    }

    #[test]
    fn project() {
        let o = 0.0;
        let x = 5.101;
        let y = 6.201;

        assert_eq!(Point::new(x, y).project(Direction::Up), Point::new(o, -y));
        assert_eq!(Point::new(x, y).project(Direction::Down), Point::new(o, y));
        assert_eq!(Point::new(x, y).project(Direction::Left), Point::new(-x, o));
        assert_eq!(Point::new(x, y).project(Direction::Right), Point::new(x, o));
    }

    #[test]
    fn add() {
        let point1 = Point::new(1.76, 3.12);
        let point2 = Point::new(2.11, 1.22);
        let point3 = Point::new(3.87, 4.34);

        assert_eq!(point1.add(point2), point3);
    }
}
