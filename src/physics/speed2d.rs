use super::{Duration, Point, Speed};
use std::{
    fmt::{Debug, Error, Formatter},
    ops::Mul,
};

/// A `Speed2D` type to represent `Speed`s on both X and Y axis.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Speed2D {
    /// The horizontal `Speed`.
    pub horizontal: Speed,
    /// The vertical `Speed`.
    pub vertical:   Speed,
}

impl Speed2D {
    /// Creates a new `Speed2D` with the specified horizontal and vertical
    /// `Speed`s.
    pub fn new(horizontal: impl Into<Speed>, vertical: impl Into<Speed>) -> Self {
        Self {
            horizontal: horizontal.into(),
            vertical:   vertical.into(),
        }
    }

    /// Multiplies a `Speed2D` by a `Duration` to produce the traveled `Point`.
    pub fn mul_duration(&self, rhs: Duration) -> Point {
        Point::new(self.horizontal * rhs, self.vertical * rhs)
    }
}

impl<T: Into<Speed>, U: Into<Speed>> From<(T, U)> for Speed2D {
    fn from((horizontal, vertical): (T, U)) -> Self {
        Self::new(horizontal, vertical)
    }
}

impl<T: From<Speed>, U: From<Speed>> From<Speed2D> for (T, U) {
    fn from(speed: Speed2D) -> Self {
        (speed.horizontal.into(), speed.vertical.into())
    }
}

impl Mul<Duration> for Speed2D {
    type Output = Point;

    fn mul(self, rhs: Duration) -> Point {
        self.mul_duration(rhs)
    }
}

impl Debug for Speed2D {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.horizontal, self.vertical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let horizontal = Speed::from_per_sec(1.0);
        let vertical = Speed::from_per_sec(2.0);

        assert_eq!(Speed2D::new(horizontal, vertical), Speed2D {
            horizontal,
            vertical,
        });
    }

    #[test]
    fn mul_duration() {
        let horizontal = Speed::from_per_sec(5.0);
        let vertical = Speed::from_per_sec(3.0);
        let duration = Duration::from_secs(2);

        assert_eq!(
            Speed2D::new(horizontal, vertical).mul_duration(duration),
            Point::new(10.0, 6.0)
        );
    }
}
