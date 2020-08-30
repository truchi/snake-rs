#[derive(Debug)]
pub struct Stepper {
    steps: Vec<u64>,
    step:  u64,
    index: usize,
}

impl Stepper {
    pub fn new(cells: u64, frames: u64) -> Self {
        let steps = to_steps(cells, frames);
        let step = 0;
        let index = 0;

        Self { steps, step, index }
    }

    pub fn step(&mut self) -> u64 {
        let mut steps = 0;

        // We may have `0`s in `steps` before the actual waiting integer
        while self.should_retick() {
            steps += 1;
            self.tick();
            // NOTE: assuming `steps` does not contains `0`s only,
            // otherwise we loop indefinitely!
        }

        // Stepping?
        if self.step == 0 {
            loop {
                steps += 1;
                self.tick();

                // We then may see others `0`s in `steps`
                // Here we do check if we went back to the beginning of the array
                if self.index == 0 || !self.should_retick() {
                    break;
                }
            }
        } else {
            // No step to take, will see in the next call
            self.tick();
        }

        steps
    }

    fn tick(&mut self) {
        self.step += 1;

        if self.step >= self.steps[self.index] {
            self.step = 0;
            self.index += 1;
            self.index %= self.steps.len();
        }
    }

    fn should_retick(&self) -> bool {
        self.steps[self.index] < 1
    }
}

fn to_steps(cells: u64, frames: u64) -> Vec<u64> {
    assert!(cells > 0, "Cells cannot be 0");
    assert!(frames > 0, "Frames cannot be 0");

    let mut vec = Vec::with_capacity(cells as usize);

    if frames % cells == 0 {
        for _ in 0..cells {
            vec.push(frames / cells);
        }

        return vec;
    }

    let lower = frames / cells;
    let upper = lower + 1;
    let upper_total = frames - (cells * lower);
    let lower_total = cells - upper_total;

    let (ratio, more, less, less_total) = if upper_total > lower_total {
        (upper_total / lower_total, upper, lower, lower_total)
    } else {
        (lower_total / upper_total, lower, upper, upper_total)
    };

    for _ in 0..less_total {
        vec.push(less);
        for _ in 0..ratio {
            vec.push(more);
        }
    }

    while (vec.len() as u64) < cells {
        vec.push(more);
    }

    vec
}

pub fn debug_steps(cells: u64, frames: u64) {
    use crossterm::style::{Color, SetForegroundColor};

    let color;

    if cells > frames {
        color = Color::Red;
    } else if frames % cells == 0 {
        color = Color::Yellow;
    } else {
        color = Color::Blue;
    }

    let steps = to_steps(cells, frames);
    let len = steps.len() as u64;
    let sum = steps.iter().fold(0, |sum, i| sum + i);
    let len_color = if len == cells {
        Color::Green
    } else {
        Color::Red
    };
    let sum_color = if sum == frames {
        Color::Green
    } else {
        Color::Red
    };

    println!(
        "{}{:3} cells, {:3} frames => {:?} {}(sum: {:2}/{:2}) {}(len: {:2}/{:2}){}",
        SetForegroundColor(color),
        cells,
        frames,
        steps,
        SetForegroundColor(sum_color),
        sum,
        frames,
        SetForegroundColor(len_color),
        len,
        cells,
        SetForegroundColor(Color::Reset),
    );
}

pub fn debug_steps_range(range: std::ops::Range<u64>) {
    for cells in range.clone() {
        for frames in range.clone() {
            debug_steps(cells, frames);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn to_steps_cells_0_panic() {
        to_steps(0, 10);
    }

    #[test]
    #[should_panic]
    fn to_steps_frames_0_panic() {
        to_steps(10, 0);
    }

    #[test]
    fn to_steps_test() {
        let range = 1..200;

        for cells in range.clone() {
            for frames in range.clone() {
                let steps = to_steps(cells, frames);
                let len = steps.len() as u64;
                let sum = steps.iter().fold(0, |sum, i| sum + i);
                assert_eq!(
                    len, cells,
                    "Length != cells ({} cells, {} frames)",
                    cells, frames
                );
                assert_eq!(
                    sum, frames,
                    "Sum != frames ({} cells, {} frames)",
                    cells, frames
                );
            }
        }
    }

    #[test]
    #[should_panic]
    fn stepper_cells_0_panic() {
        Stepper::new(0, 10);
    }

    #[test]
    #[should_panic]
    fn stepper_frames_0_panic() {
        Stepper::new(10, 0);
    }

    #[test]
    fn stepper() {
        let range = 1..200;
        let runs = 3;

        for cells in range.clone() {
            for frames in range.clone() {
                let mut stepper = Stepper::new(cells, frames);
                let mut stepped = 0;

                for run in 1..=runs {
                    for _ in 0..frames {
                        stepped += stepper.step();
                    }

                    assert_eq!(
                        stepped,
                        run * cells,
                        "{} cells, {} frames, run {}/{}",
                        cells,
                        frames,
                        run,
                        runs
                    );
                }
            }
        }
    }
}
