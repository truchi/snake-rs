use super::{Coord2D, Distance};

/// A `Point` type (`Coord2D<Distance>`).
pub type Point = Coord2D<Distance>;

impl Point {
    /// Returns the euclidian `Distance` from the origin.
    pub fn length(&self) -> Distance {
        Distance::sqrt(self.x * self.x + self.y * self.y)
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
}
