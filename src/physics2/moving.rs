use super::Point;
use std::time::Duration;

pub trait Moving {
    fn r#move(&mut self, delta: Duration) -> Point;
}
