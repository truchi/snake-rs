use crate::{
    events::{poll, Event, KeyCode, KeyEvent},
    CONTINUE,
    CONTINUE_DELAY,
    CONTINUE_HEIGHT,
    CONTINUE_WIDTH,
    SNAKE,
    SNAKE_BLINK_TIME,
    SNAKE_HEIGHT,
    SNAKE_WIDTH,
    WELCOME,
    WELCOME_FPS,
    WELCOME_HEIGHT,
    WELCOME_WIDTH,
};
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

#[derive(Debug)]
pub enum WelcomeAction {
    Continue,
    Quit,
}

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

    pub fn is_snake_blinking(&self) -> bool {
        if let State::BlinkingSnake(_) = self.state {
            true
        } else {
            false
        }
    }

    pub fn snake_y(&self) -> u16 {
        self.snake_y
    }

    pub fn show(&mut self) -> WelcomeAction {
        loop {
            self.welcome_frame();
            self.snake_frame();
            self.continue_frame();
            self.out.flush().unwrap();

            if let Some(Event::Key(KeyEvent { code, .. })) = poll() {
                if code == KeyCode::Esc {
                    return WelcomeAction::Quit;
                } else {
                    return WelcomeAction::Continue;
                }
            }

            sleep(Duration::from_millis(1000 / WELCOME_FPS));
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
