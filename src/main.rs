// mod consts;
// mod direction;
// mod events;
// mod game;
// mod menu;
// mod position;
// mod screen;
// mod snake;
// mod welcome;
// mod world;
//
// use consts::*;
// use direction::*;
// use events::*;
// use game::*;
// use menu::*;
// use position::*;
// use screen::*;
// use snake::*;
// use welcome::*;
// use world::*;

fn main() {
    fn print_one(cps: u32, fps: u32) {
        use crossterm::style::{Color, SetForegroundColor};
        let color;

        if cps > fps {
            color = Color::Red;
        } else if fps % cps == 0 {
            color = Color::Yellow;
        } else {
            color = Color::Blue;
        }

        let one = yop(cps, fps);
        let len = one.len() as u32;
        let sum = one.iter().fold(0, |sum, i| sum + i);
        let len_color = if len == cps { Color::Green } else { Color::Red };
        let sum_color = if sum == fps { Color::Green } else { Color::Red };

        if false {
            println!(
                "{}{:3} FPS {:3} CPS ==> {:.3} CPF / {:.3} FPC {:?} {}(sum: {}/{}) {}(len: {}/{}){}",
                SetForegroundColor(color),
                fps,
                cps,
                cps as f64 / fps as f64,
                fps as f64 / cps as f64,
                one,
                SetForegroundColor(sum_color),
                sum,
                fps,
                SetForegroundColor(len_color),
                len,
                cps,
                SetForegroundColor(Color::Reset),
            );
        }

        if true {
            debug_assert_eq!(
                sum, fps,
                "Sum does not equal frames (cells {}, frames: {}, sum: {})",
                cps, fps, sum
            );
            debug_assert_eq!(
                len, cps,
                "Len does not equal cells (cells {}, frames: {}, len: {})",
                cps, fps, len
            );
        }
    }

    fn print_range(r: std::ops::RangeInclusive<u32>) {
        for fps in r.clone() {
            for cps in r.clone() {
                print_one(cps, fps);
            }
        }
    }

    print_range(1..=20);

    fn yop(cells: u32, frames: u32) -> Vec<u32> {
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
        debug_assert_eq!(lower_total + upper_total, cells);
        debug_assert_eq!(lower_total * lower + upper_total * upper, frames);

        let (ratio, less, less_total, more, more_total) = if upper_total > lower_total {
            (
                upper_total / lower_total,
                lower,
                lower_total,
                upper,
                upper_total,
            )
        } else {
            (
                lower_total / upper_total,
                upper,
                upper_total,
                lower,
                lower_total,
            )
        };

        for _ in 0..less_total {
            vec.push(less);
            for _ in 0..ratio {
                vec.push(more);
            }
        }

        while (vec.len() as u32) < cells {
            vec.push(more);
        }

        vec
    }

    // Game::new().init();
}
