use super::{Direction, Speed};
use std::ops::Rem;

/// A `Speed2D` type to represent `Speed`s on both X and Y axis.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Speed2D {
    /// The horizontal `Speed`.
    pub horizontal: Speed,
    /// The vertical `Speed`.
    pub vertical: Speed,
}

impl Speed2D {
    /// Creates a new `Speed2D` with the specified horizontal and vertical `Speed`s.
    pub fn new(horizontal: impl Into<Speed>, vertical: impl Into<Speed>) -> Self {
        Self {
            horizontal: horizontal.into(),
            vertical: vertical.into(),
        }
    }

    /// Returns the `Speed` on the specified `Direction`
    pub fn on_direction(self, direction: impl Into<Direction>) -> Speed {
        if direction.into().is_horizontal() {
            self.horizontal
        } else {
            self.vertical
        }
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

/// Calls `Speed2D::on_direction`
impl<T: Into<Direction>> Rem<T> for Speed2D {
    type Output = Speed;

    fn rem(self, rhs: T) -> Speed {
        self.on_direction(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(
            Speed2D::new(Speed::from_units_per_sec(1), Speed::from_units_per_sec(2)),
            Speed2D {
                horizontal: Speed::from_units_per_sec(1),
                vertical: Speed::from_units_per_sec(2)
            }
        );
    }

    #[test]
    fn on_direction() {
        let horizontal = Speed::from_units_per_sec(3);
        let vertical = Speed::from_units_per_sec(4);
        let speed2d = Speed2D::new(horizontal, vertical);

        assert_eq!(speed2d.on_direction(Direction::Up), vertical);
        assert_eq!(speed2d.on_direction(Direction::Down), vertical);
        assert_eq!(speed2d.on_direction(Direction::Left), horizontal);
        assert_eq!(speed2d.on_direction(Direction::Right), horizontal);
    }
}
