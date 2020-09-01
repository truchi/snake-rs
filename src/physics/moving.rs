use super::{Direction, Point, Speed2D};
use std::time::Duration;

pub trait Moving {
    fn position(&mut self) -> &mut Point;
    fn direction(&self) -> Direction;
    fn speed(&self) -> Speed2D;

    fn update(&mut self, delta: Duration) -> Option<Point> {
        let mut ret = None;
        let direction = self.direction();
        let speed = self.speed();
        let position = self.position();

        let new_position = *position + (speed * delta) % direction;

        if new_position.trunc() != position.trunc() {
            ret = Some(new_position);
        }

        *position = new_position;
        ret
    }
}
