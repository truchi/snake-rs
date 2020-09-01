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

impl Direction {
    /// Returns the opposite `Direction`.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        self.opposite()
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
    fn opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }
}
