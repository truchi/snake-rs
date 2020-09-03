use super::Point;
use std::{
    fmt::{Debug, Error, Formatter},
    ops::Neg,
};

/// A type to represent the four `Direction`s.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    /// The up `Direction`.
    Up,
    /// The down `Direction`.
    Down,
    /// The left `Direction`.
    Left,
    /// The right `Direction`.
    Right,
}

impl Into<Point> for Direction {
    fn into(self) -> Point {
        match self {
            Self::Up => Point::new(0.0, -1.0),
            Self::Down => Point::new(0.0, 1.0),
            Self::Left => Point::new(-1.0, 0.0),
            Self::Right => Point::new(1.0, 0.0),
        }
    }
}

/// Returns the opposite `Direction`.
impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Up => write!(f, "↑"),
            Self::Down => write!(f, "↓"),
            Self::Left => write!(f, "←"),
            Self::Right => write!(f, "→"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn into_point() {
        let into = <Direction as Into<Point>>::into;

        assert_eq!(into(Direction::Up), Point::new(0.0, -1.0));
        assert_eq!(into(Direction::Down), Point::new(0.0, 1.0));
        assert_eq!(into(Direction::Left), Point::new(-1.0, 0.0));
        assert_eq!(into(Direction::Right), Point::new(1.0, 0.0));
    }

    #[test]
    fn neg() {
        assert_eq!(-Direction::Up, Direction::Down);
        assert_eq!(-Direction::Down, Direction::Up);
        assert_eq!(-Direction::Left, Direction::Right);
        assert_eq!(-Direction::Right, Direction::Left);
    }
}
