use crate::{Direction, Screen, Snake};

#[derive(Debug)]
pub struct World {
    snake:  Snake,
    screen: Screen,
}

impl World {
    pub fn new() -> Self {
        let screen = Screen::new();
        let snake = Snake::new((0, 0), Direction::Right);

        Self { snake, screen }
    }

    pub fn r#move(&mut self) {
        let moved = self.snake.slither(self.screen.bounds());
        match moved {
            Ok(_) => (),
            Err(err) => {
                println!("BANG!!! {:#?}", err);
                panic!();
            }
        }
        println!("{:#?}", self.snake);

        ()
    }
}
