use crate::{
    events::{poll_all, Event, KeyCode, KeyEvent},
    MenuAction,
    Screen,
    WelcomeAction,
    World,
    FPS,
};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

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

        match self.screen.welcome() {
            WelcomeAction::Quit => {
                return;
            }
            WelcomeAction::Continue => {}
        }

        match self.screen.menu() {
            MenuAction::Quit => {
                return;
            }
            MenuAction::Level(_) => {
                self.play();
            }
        }
    }

    pub fn play(&self) {
        let mut world = World::new(self.screen.size());
        let mpf = Duration::from_millis(1000 / FPS);

        'game_loop: loop {
            let start = Instant::now();

            let events = poll_all();
            for event in events {
                match event {
                    Event::Resize(..) => {}
                    Event::Mouse(_) => {}
                    Event::Key(KeyEvent { code, .. }) =>
                        if let KeyCode::Esc = code {
                            break 'game_loop;
                        } else {
                            world.handle(code);
                        },
                }
            }

            world.update();
            world.render();

            let elapsed = start.elapsed();
            debug_assert!(elapsed < mpf);

            sleep(mpf - elapsed);
        }
    }
}
