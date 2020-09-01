use std::ops::Not;

/// A type to represent the four `Direction`s.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Returns `true` if the `Direction` is horizontal.
    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            _ => false,
        }
    }

    /// Returns `true` if the `Direction` is heading towards the positive side
    /// of the axis.
    pub fn is_positive(&self) -> bool {
        match self {
            Direction::Right | Direction::Down => true,
            _ => false,
        }
    }
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self {
        self.opposite()
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

    #[test]
    fn is_horizontal() {
        assert_eq!(Direction::Up.is_horizontal(), false);
        assert_eq!(Direction::Down.is_horizontal(), false);
        assert_eq!(Direction::Left.is_horizontal(), true);
        assert_eq!(Direction::Right.is_horizontal(), true);
    }

    #[test]
    fn is_positive() {
        assert_eq!(Direction::Up.is_positive(), false);
        assert_eq!(Direction::Down.is_positive(), true);
        assert_eq!(Direction::Left.is_positive(), false);
        assert_eq!(Direction::Right.is_positive(), true);
    }
}
