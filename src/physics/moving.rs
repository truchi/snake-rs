use super::{Coord2D, Direction, Point, Speed2D};
use std::time::Duration;

pub trait Moving {
    fn r#move(&mut self, delta: Duration);
}

// TODO: test
impl Moving for (&mut Point, Speed2D) {
    fn r#move(&mut self, duration: Duration) {
        let (position, speed) = self;

        **position += *speed * duration.into();
    }
}

// TODO: test
impl Moving for (&mut Point, Speed2D, Direction) {
    fn r#move(&mut self, duration: Duration) {
        let (position, speed, direction) = self;
        let direction: Point = (*direction).into();
        let duration: Coord2D<Duration> = duration.into();

        **position += (*speed * duration) * direction;
    }
}
