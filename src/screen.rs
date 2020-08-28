use crate::{Menu, MenuAction, Position, Welcome, WelcomeAction};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    queue,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        size,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
        SetTitle,
    },
};
use std::io::{stdout, Stdout, Write};

const ALTERNATE_SCREEN: bool = true;
const RAW_MODE: bool = true;

pub const TITLE: &str = "🐍🐍👅";

pub const WELCOME_FPS: u64 = 100;
pub const MENU_FPS: u64 = 20;

pub const fn millis_to_frames(millis: u64, fps: u64) -> u64 {
    fps * millis / 1000
}

/// Half cycle, in frames
pub const SNAKE_BLINK_TIME: u64 = millis_to_frames(200, WELCOME_FPS);

/// After "snake" started blinking, in frames
pub const CONTINUE_DELAY: u64 = millis_to_frames(1000, WELCOME_FPS);

pub const WELCOME: [&str; 11] = [
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
pub const WELCOME_WIDTH: u16 = WELCOME[0].len() as u16;
pub const WELCOME_HEIGHT: u16 = WELCOME.len() as u16;

pub const SNAKE: [&str; 5] = [
    r"🐍🐍👅  🐍    👅      🐍      🐍  👅  🐍🐍👅",
    r"🐍      🐍🐍  🐍     🐍🐍     🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍 🐍 🐍    🐍  🐍    🐍🐍    🐍🐍  ",
    r"    🐍  🐍  🐍🐍   🐍 🐍 🐍   🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍    🐍  🐍      👅  🐍  🐍  🐍🐍🐍",
];
pub const SNAKE_WIDTH: u16 = SNAKE[0].len() as u16 - 19; // Adjust because weird chars
pub const SNAKE_HEIGHT: u16 = SNAKE.len() as u16;

pub const CONTINUE: [&str; 2] = [
    "       [Press ESC to quit]      ",
    "[Press any other key to continue]",
];
pub const CONTINUE_WIDTH: u16 = CONTINUE[0].len() as u16;
pub const CONTINUE_HEIGHT: u16 = CONTINUE.len() as u16;

pub const KEYS: [&str; 9] = [
    "[", "↲", " play ", "↑", " prev ", "↓", " next ", "ESC", " quit]",
];

#[derive(Debug)]
pub struct Screen {
    out:             Stdout,
    entered:         bool,
    width:           u16,
    height:          u16,
    welcome:         Welcome,
    menu:            Menu,
    menu_shown_once: bool,
}

impl Screen {
    pub fn new() -> Self {
        let entered = false;
        let out = stdout();
        let (width, height) = size().expect("Cannot get terminal size");
        let welcome = Welcome::new(width, height);
        let menu = Menu::new(width, height);
        let menu_shown_once = false;

        Self {
            out,
            entered,
            width,
            height,
            welcome,
            menu,
            menu_shown_once,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn size(&self) -> Position {
        (self.width as i32, self.height as i32).into()
    }

    pub fn clear(&mut self) {
        execute!(self.out, Clear(ClearType::All)).unwrap();
    }

    pub fn welcome(&mut self) -> WelcomeAction {
        self.clear();
        let action = self.welcome.show();
        self.clear();

        action
    }

    pub fn menu(&mut self) -> MenuAction {
        let snake_y_anim = if !self.menu_shown_once && self.welcome.is_snake_blinking() {
            Some(self.welcome.snake_y())
        } else {
            None
        };

        self.clear();
        let action = self.menu.show(snake_y_anim);
        self.clear();

        self.menu_shown_once = true;

        action
    }

    pub fn enter(&mut self) {
        if self.entered {
            return;
        }

        if RAW_MODE {
            enable_raw_mode().expect("Cannot enable raw mode");
        }

        queue!(self.out, Hide).expect("Cannot hide cursor");

        if ALTERNATE_SCREEN {
            queue!(self.out, EnterAlternateScreen).expect("Cannot enter alternate screen");
        }

        queue!(self.out, SetTitle(TITLE)).expect("Cannot set terminal title");
        self.out.flush().expect("Cannot flush");

        self.entered = true;
    }

    pub fn leave(&mut self) {
        if !self.entered {
            return;
        }

        if ALTERNATE_SCREEN {
            queue!(self.out, LeaveAlternateScreen).expect("Cannot leave alternate screen");
        }

        queue!(self.out, Show).expect("Cannot show cursor");

        queue!(self.out, SetTitle("")).expect("Cannot restore terminal title");
        self.out.flush().expect("Cannot flush");

        if RAW_MODE {
            disable_raw_mode().expect("Cannot disable raw mode");
        }

        self.entered = false;
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        Screen::leave(self);
    }
}
