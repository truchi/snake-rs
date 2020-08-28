use crate::events::{poll, Event};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Stdout, Write},
    thread::sleep,
    time::Duration,
};

const FPS: u64 = 100;

const fn millis_to_frames(millis: u64) -> u64 {
    FPS * millis / 1000
}

/// Half cycle, in frames
const SNAKE_BLINK_TIME: u64 = millis_to_frames(200);

/// After "snake" started blinking, in frames
const CONTINUE_DELAY: u64 = millis_to_frames(1000);

const WELCOME: [&str; 11] = [
    r" __      __       .__                               ",
    r"/  \    /  \ ____ |  |   ____  ____   _____   ____  ",
    r"\   \/\/   // __ \|  | _/ ___\/  _ \ /     \_/ __ \ ",
    r" \        /\  ___/|  |_\  \__(  <_> )  Y Y  \  ___/ ",
    r"  \__/\  /  \___  >____/\___  >____/|__|_|  /\___  >",
    r"       \/       \/          \/            \/     \/ ",
    r"                     __                             ",
    r"                   _/  |_  ____                     ",
    r"                   \   __\/  _ \                    ",
    r"                    |  | (  <_> )                   ",
    r"                    |__|  \____/                    ",
];
const WELCOME_WIDTH: u16 = WELCOME[0].len() as u16;
const WELCOME_HEIGHT: u16 = WELCOME.len() as u16;

const SNAKE: [&str; 5] = [
    r"🐍🐍👅  🐍    👅      🐍      🐍  👅  🐍🐍👅",
    r"🐍      🐍🐍  🐍     🐍🐍     🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍 🐍 🐍    🐍  🐍    🐍🐍    🐍🐍  ",
    r"    🐍  🐍  🐍🐍   🐍 🐍 🐍   🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍    🐍  🐍      👅  🐍  🐍  🐍🐍🐍",
];
const SNAKE_WIDTH: u16 = SNAKE[0].len() as u16 - 19; // Adjust because weird chars
const SNAKE_HEIGHT: u16 = SNAKE.len() as u16;

const CONTINUE: [&str; 1] = ["[Press any key to continue]"];
const CONTINUE_WIDTH: u16 = CONTINUE[0].len() as u16;
const CONTINUE_HEIGHT: u16 = CONTINUE.len() as u16;

#[derive(Debug)]
enum State {
    SlidingWelcome(u16),
    BlinkingSnake(u64),
}

#[derive(Debug)]
pub struct Welcome {
    out:        Stdout,
    state:      State,
    width:      u16,
    height:     u16,
    welcome_x:  u16,
    welcome_y:  u16,
    snake_x:    u16,
    snake_y:    u16,
    continue_x: u16,
    continue_y: u16,
}

impl Welcome {
    pub fn new(width: u16, height: u16) -> Self {
        let out = stdout();
        let state = State::SlidingWelcome(0);

        let welcome_x = (width - WELCOME_WIDTH) / 2;
        let welcome_y = (height - WELCOME_HEIGHT - 1 - SNAKE_HEIGHT - 1 - CONTINUE_HEIGHT) / 2;
        let snake_x = (width - SNAKE_WIDTH) / 2;
        let snake_y = welcome_y + 1 + WELCOME_HEIGHT;
        let continue_x = (width - CONTINUE_WIDTH) / 2;
        let continue_y = snake_y + 1 + SNAKE_HEIGHT;

        Self {
            out,
            state,
            width,
            height,
            welcome_x,
            welcome_y,
            snake_x,
            snake_y,
            continue_x,
            continue_y,
        }
    }

    pub fn size() -> (u16, u16) {
        (
            WELCOME_WIDTH.max(SNAKE_WIDTH).max(CONTINUE_WIDTH),
            WELCOME_HEIGHT + 1 + SNAKE_HEIGHT + 1 + CONTINUE_HEIGHT,
        )
    }

    pub fn show(&mut self) {
        loop {
            self.welcome_frame();
            self.snake_frame();
            self.continue_frame();
            self.out.flush().unwrap();

            if let Some(Event::Key(_)) = poll() {
                break;
            }

            sleep(Duration::from_millis(1000 / FPS));
            self.next_state();
        }
    }

    fn welcome_frame(&mut self) {
        if let State::SlidingWelcome(x) = self.state {
            let i = WELCOME_WIDTH.min(x);

            for y in 0..WELCOME_HEIGHT {
                queue!(
                    self.out,
                    MoveTo(self.width - x, self.welcome_y + y),
                    Clear(ClearType::CurrentLine),
                    Print(&WELCOME[y as usize][0..i as usize]),
                )
                .unwrap();
            }
        }
    }

    fn snake_frame(&mut self) {
        if let State::BlinkingSnake(x) = self.state {
            let cycle = x % (SNAKE_BLINK_TIME * 2);
            let show = cycle == 0;
            let hide = cycle == SNAKE_BLINK_TIME;

            if show || hide {
                for y in 0..SNAKE_HEIGHT {
                    queue!(self.out, MoveTo(self.snake_x, self.snake_y + y)).unwrap();
                    if show {
                        queue!(self.out, Print(&SNAKE[y as usize])).unwrap();
                    } else {
                        queue!(self.out, Clear(ClearType::CurrentLine)).unwrap();
                    }
                }
            }
        }
    }

    fn continue_frame(&mut self) {
        if let State::BlinkingSnake(x) = self.state {
            if x == CONTINUE_DELAY {
                for y in 0..CONTINUE_HEIGHT {
                    queue!(
                        self.out,
                        MoveTo(self.continue_x, self.continue_y + y),
                        Print(&CONTINUE[y as usize])
                    )
                    .unwrap();
                }
            }
        }
    }

    fn next_state(&mut self) {
        match self.state {
            State::SlidingWelcome(x) =>
                if x > self.width - self.welcome_x {
                    self.state = State::BlinkingSnake(0);
                } else {
                    self.state = State::SlidingWelcome(x + 1);
                },
            State::BlinkingSnake(x) => self.state = State::BlinkingSnake(x + 1),
        }
    }
}
