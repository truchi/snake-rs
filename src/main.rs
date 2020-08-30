pub mod consts;
pub mod events;
pub mod game;
pub mod geometry;
pub mod physics;
pub mod screen;

fn main() {
    // crate::physics::debug_steps_range(1..21);

    let cells = 2;
    let frames = 1;
    let mut stepper = crate::physics::Stepper::new(cells, frames);
    for _ in 0..3 {
        println!();
        println!("--");
        for i in 0..frames {
            println!("--> i: {}, stepped: {}", i, stepper.step());
            println!();
        }
    }

    // game::Game::new().init();
}
