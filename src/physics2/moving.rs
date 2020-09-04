use super::{Point, Speed2D};
use std::time::Duration;

pub trait Moving {
    fn r#move(&mut self, delta: Duration);
}

impl Moving for (&mut Point, Speed2D) {
    fn r#move(&mut self, duration: Duration) {
        let (position, speed) = self;
        **position += *speed * duration.into();
    }
}
