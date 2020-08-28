use crate::Screen;

#[derive(Debug)]
pub struct Game {
    screen: Screen,
}

impl Game {
    pub fn new() -> Self {
        let screen = Screen::new();

        Self { screen }
    }

    pub fn init(&mut self) {
        self.screen.enter();
        self.screen.welcome();
        self.screen.menu();
    }
}
