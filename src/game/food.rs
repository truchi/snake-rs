use crate::physics::Point;

#[derive(Debug)]
pub struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: impl Into<Point>) -> Self {
        Self {
            position: position.into(),
        }
    }
}
