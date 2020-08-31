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
    pub fn new(x: Distance, y: Distance) -> Self {
        Self { x, y }
    }

    /// Creates a new `Point` from its `x`/`y` coordinates as `u16`.
    pub fn from_units(x: u16, y: u16) -> Self {
        Self {
            x: Distance::from_units(x),
            y: Distance::from_units(y),
        }
    }

    /// Returns the coordinates of this `Point` as *whole* units.
    pub fn as_units(self) -> (u16, u16) {
        (self.x.as_units(), self.y.as_units())
    }

    /// Adds a `Distance` to each of the coordinates of a `Point`.
    pub fn add_distance(self, rhs: Distance) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl From<(Distance, Distance)> for Point {
    fn from((x, y): (Distance, Distance)) -> Self {
        Self::new(x, y)
    }
}

impl Into<(Distance, Distance)> for Point {
    fn into(self) -> (Distance, Distance) {
        (self.x, self.x)
    }
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self {
        Self::from_units(x, y)
    }
}

impl Into<(u16, u16)> for Point {
    fn into(self) -> (u16, u16) {
        self.as_units()
    }
}

impl Add<Distance> for Point {
    type Output = Self;

    fn add(self, rhs: Distance) -> Self {
        self.add_distance(rhs)
    }
}

impl AddAssign<Distance> for Point {
    fn add_assign(&mut self, rhs: Distance) {
        *self = self.add(rhs);
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
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
    fn from_units() {
        assert_eq!(
            Point::from_units(11, 23),
            Point::new(Distance::new(11, 0), Distance::new(23, 0))
        );
    }
}
