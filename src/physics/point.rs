use super::{Coord2D, Direction, Distance};

/// A `Point` type (`Coord2D<Distance>`).
pub type Point = Coord2D<Distance>;

impl Point {
    /// Rounds the coordinates.
    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    /// Truncates the coordinates.
    pub fn trunc(&self) -> Self {
        Self::new(self.x.trunc(), self.y.trunc())
    }

    /// Returns the euclidian `Distance` from the origin.
    pub fn length(&self) -> Distance {
        Distance::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl From<Direction> for Point {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn length() {
        assert_eq!(Point::new(3.0, 4.0).length(), 5.0);
    }

    // TODO: round, trunc

    #[test]
    fn from_direction() {
        let from = Point::from;

        assert_eq!(from(Direction::Up), Point::new(0.0, -1.0));
        assert_eq!(from(Direction::Down), Point::new(0.0, 1.0));
        assert_eq!(from(Direction::Left), Point::new(-1.0, 0.0));
        assert_eq!(from(Direction::Right), Point::new(1.0, 0.0));
    }
}
