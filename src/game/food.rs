use crate::physics::{Duration, Moving, Path, PathFragment, Point};
use crossterm::cursor::MoveTo;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Food<T: Iterator<Item = PathFragment>> {
    initial:      Point,
    pub position: Point,
    path:         Path<T>,
}

impl<T: Iterator<Item = PathFragment>> Food<T> {
    pub fn new(position: impl Into<Point>, path: T) -> Self {
        let position = position.into();

        Self {
            initial: position,
            position,
            path: Path::new(path),
        }
    }
}

impl<T: Iterator<Item = PathFragment>> Moving for Food<T> {
    fn r#move(&mut self, duration: Duration) {
        self.path.r#move(duration);
        self.position = self.initial + self.path.position;
    }
}

impl<T: Iterator<Item = PathFragment>> Display for Food<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let position = self.position.round();

        write!(f, "{}🦀", MoveTo(position.x as u16 * 2, position.y as u16))
    }
}
