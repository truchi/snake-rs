use super::Distance;
use std::{
    fmt::{Debug, Error, Formatter},
    ops::{Add, AddAssign},
};

/// A `Point` type to represent a position in cartesian space.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Point {
    /// The `Distance` to origin on the X axis.
    pub x: Distance,
    /// The `Distance` to origin on the Y axis.
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

    /// Adds a `Distance` to each of the coordinates of a `Point`.
    pub fn add_distance(&self, rhs: impl Into<Distance>) -> Self {
        let rhs = rhs.into();

        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
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

impl<T: Into<Distance>> Add<T> for Point {
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        self.add_distance(rhs)
    }
}

impl<T: Into<Distance>> AddAssign<T> for Point {
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
        assert_eq!(
            Point::new(Distance::new(1, 23), Distance::new(4, 56)),
            Point {
                x: Distance::new(1, 23),
                y: Distance::new(4, 56),
            }
        );
    }

    #[test]
    fn add_distance() {
        assert_eq!(
            Point::new(Distance::new(1, 760), Distance::new(3, 121))
                .add_distance(Distance::new(9, 122)),
            Point::new(Distance::new(10, 882), Distance::new(12, 243))
        );
    }
}
