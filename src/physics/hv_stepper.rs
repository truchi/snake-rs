use super::Stepper;
use crate::geometry::Direction;

#[derive(Debug)]
pub struct HVStepper {
    horizontal: Stepper,
    vertical:   Stepper,
}

impl HVStepper {
    pub fn new(horizontal_speed: u64, vertical_speed: u64) -> Self {
        let horizontal = Stepper::new(horizontal_speed);
        let vertical = Stepper::new(vertical_speed);

        Self {
            horizontal,
            vertical,
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        if direction.is_horizontal() {
            return self.horizontal.step();
        } else {
            return self.vertical.step();
        }
    }
}
