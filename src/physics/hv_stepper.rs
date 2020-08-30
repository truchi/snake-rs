use super::Stepper;
use crate::geometry::Direction;

#[derive(Debug)]
pub struct HVStepper {
    horizontal: Stepper,
    vertical:   Stepper,
}

impl HVStepper {
    pub fn new(horizontal_speed: u64, vertical_speed: u64, frames: u64) -> Self {
        let horizontal = Stepper::new(horizontal_speed, frames);
        let vertical = Stepper::new(vertical_speed, frames);

        Self {
            horizontal,
            vertical,
        }
    }

    pub fn step(&mut self, direction: Direction) -> u64 {
        return if direction.is_horizontal() {
            self.horizontal.step()
        } else {
            self.vertical.step()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn hv_stepper() {
        let h_cells = 2;
        let v_cells = 4;
        let frames = 8;
        let runs = 3;

        let mut stepper = HVStepper::new(h_cells, v_cells, frames);
        let mut h_stepped = 0;
        let mut v_stepped = 0;

        for run in 1..=runs {
            for i in 0..(2 * frames) {
                if i % 2 == 0 {
                    h_stepped += stepper.step(Direction::Left);
                } else {
                    v_stepped += stepper.step(Direction::Up);
                }
            }

            assert_eq!(
                h_stepped,
                run * h_cells,
                "{} h_cells, {} frames, run {}/{}",
                h_cells,
                frames,
                run,
                runs
            );
            assert_eq!(
                v_stepped,
                run * v_cells,
                "{} v_cells, {} frames, run {}/{}",
                v_cells,
                frames,
                run,
                runs
            );
            assert_eq!(
                h_stepped + v_stepped,
                run * (h_cells + v_cells),
                "{} cells, {} frames, run {}/{}",
                h_cells + v_cells,
                frames,
                run,
                runs
            );
        }
    }
}
