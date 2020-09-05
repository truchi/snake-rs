use super::{Coord2D, Direction, Distance, Duration, Point, Speed};

/// A `Speed2D` type (`Coord2D<Speed<Distance>>`).
pub type Speed2D = Coord2D<Speed<Distance>>;

impl Speed2D {
    /// Creates a new `Speed2D` from a `per_sec` speed as `Point`.
    pub fn from_per_sec(per_sec: impl Into<Point>) -> Self {
        let per_sec = per_sec.into();

        Self::new(
            (per_sec.x, Duration::from_secs(1)).into(),
            (per_sec.y, Duration::from_secs(1)).into(),
        )
    }

    /// Returns this `Speed2D` expressed as `Point`s per second.
    pub fn as_per_sec(&self) -> Point {
        Point::new(self.x.as_per_sec(), self.y.as_per_sec())
    }
}

impl<T: Into<Point>> From<(T, Duration)> for Speed2D {
    fn from((point, duration): (T, Duration)) -> Self {
        let point = point.into();
        Self::new((point.x, duration).into(), (point.y, duration).into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_per_sec() {
        assert_eq!(
            Speed2D::from_per_sec((2.9, 3.8)),
            Speed2D::new(
                Speed::new(2.9, Duration::from_secs(1)),
                Speed::new(3.8, Duration::from_secs(1)),
            )
        );
    }

    #[test]
    fn as_per_sec() {
        assert_eq!(
            Speed2D::new(
                Speed::new(3.0, Duration::from_secs(3)),
                Speed::new(9.0, Duration::from_secs(3)),
            )
            .as_per_sec(),
            Point::new(1.0, 3.0)
        );
    }

    #[test]
    fn from_tuple() {
        assert_eq!(
            Speed2D::from(((2.1, 1.2), Duration::from_secs(2))),
            Speed2D::new(
                Speed::new(2.1, Duration::from_secs(2)),
                Speed::new(1.2, Duration::from_secs(2)),
            )
        );
    }
}
