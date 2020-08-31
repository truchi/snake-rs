/// Emits `step`s at a ratio of a rational number -- the speed
///
/// Used to move object at some speed when calling at regular intervals. The
/// speed may be greater than one, i.e. objects can move faster in space than in
/// time. When the speed is not an integer, we do our best to spread the
/// motion across time.
#[derive(Debug)]
pub struct Stepper {
    /// The sequence of `frames` to wait for each `step`
    sequence: Vec<u64>,
    /// The current `step`ping progress
    progress: u64,
    /// The current index in the sequence
    index:    usize,
}

impl Stepper {
    /// Returns a new `Stepper`
    ///
    /// # Panics
    ///
    /// Panics if `cells` or `frames` is `0`
    pub fn new(cells: u64, frames: u64) -> Self {
        let sequence = to_sequence(cells, frames);
        let progress = 0;
        let index = 0;

        Self {
            sequence,
            progress,
            index,
        }
    }

    /// Progresses the `Stepper` by one `frame` and returns the corresponding
    /// `step` count
    pub fn next(&mut self) -> u64 {
        let mut steps = 0;

        // We may have `0`s in `steps` before the actual waiting integer
        while self.should_retick() {
            steps += 1;
            self.tick();
            // NOTE: assuming `steps` does not contains `0`s only,
            // otherwise we loop indefinitely!
        }

        // Stepping?
        if self.progress == 0 {
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

    /// Progresses the inner state by one
    ///
    /// A `frame` might take multiple `tick`s!
    fn tick(&mut self) {
        // Increment progress
        self.progress += 1;

        // When reached stepping
        if self.progress >= self.sequence[self.index] {
            // Reset progress
            self.progress = 0;
            // Increment index
            self.index += 1;
            self.index %= self.sequence.len();
        }
    }

    /// Returns `true` if this `tick` is not a full `frame` (in which case we
    /// might want to `tick` again to go to the next `frame`)
    fn should_retick(&self) -> bool {
        self.sequence[self.index] < 1
    }
}

/// Sequences a rational number into a `Vec` of steps
///
/// TODO documentation
///
/// # Panics
///
/// Panics if `cells` or `frames` is `0`
fn to_sequence(cells: u64, frames: u64) -> Vec<u64> {
    assert!(cells > 0, "Cells cannot be 0");
    assert!(frames > 0, "Frames cannot be 0");

    let mut vec = Vec::with_capacity(cells as usize);

    // When `cells` is a multiple of `frames`,
    // we fill `vec` with the `frames / cell` ratio
    // User might want to reduce the fraction
    if frames % cells == 0 {
        for _ in 0..cells {
            vec.push(frames / cells);
        }

        return vec;
    }

    // When the fraction is irrational, we can express it under the form:
    // [floor(frames / cells) | ceil(frames / cells)]+
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

    let steps = to_sequence(cells, frames);
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
    fn to_sequence_cells_0_panic() {
        to_sequence(0, 10);
    }

    #[test]
    #[should_panic]
    fn to_sequence_frames_0_panic() {
        to_sequence(10, 0);
    }

    #[test]
    fn to_sequence_test() {
        let range = 1..200;

        for cells in range.clone() {
            for frames in range.clone() {
                let steps = to_sequence(cells, frames);
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
                        stepped += stepper.next();
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
