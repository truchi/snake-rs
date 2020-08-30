use crate::FPS;

#[derive(Debug)]
pub struct Stepper {
    steps: Vec<u64>,
    index: usize,
    step:  u64,
}

impl Stepper {
    pub fn new(speed: u64) -> Self {
        let steps = to_steps(speed, FPS);
        let index = 0;
        let step = 0;

        Self { steps, index, step }
    }

    pub fn step(&mut self) -> bool {
        let mut steps = false;

        if self.step % self.steps[self.index] == 0 {
            steps = true;
        }

        if self.step < self.steps[self.index] {
            self.step += 1;
        } else {
            self.step = 0;
            self.index = (self.index + 1) % self.steps.len();
        }

        steps
    }
}

fn to_steps(cells: u64, frames: u64) -> Vec<u64> {
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
