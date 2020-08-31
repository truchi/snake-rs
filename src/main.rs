//! # Snake
//!
//! The **best** Snake

// pub mod consts;
// pub mod events;
// pub mod game;
// pub mod geometry;
pub mod physics;
// pub mod screen;

fn main() {
    use physics::*;

    let fps: u64 = 10;
    let cps: u16 = 10;

    let spf = Duration::from_secs_f64(1 as f64 / fps as f64);
    // let speed = Speed::from_units_per_sec(cps);
    // let position = Point::from_units(0, 0);
    let speed = Speed::from(cps);
    let mut position = Point::from((10, 10));

    let delta_position = speed * spf;
    position += delta_position;
    // let new_position = position + delta_position;

    println!(
        "speed {:?}, spf {:?}, speed * spf = {:?} (position {:?})",
        speed, spf, delta_position, position
    );

    // game::Game::new().init();
}
