#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        if self == &Direction::Left || self == &Direction::Right {
            return true;
        }

        return false;
    }
}
