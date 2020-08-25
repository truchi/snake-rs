use crate::Position;
use crossterm::{
    cursor::{Hide, Show},
    queue,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        size,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write};

const ALTERNATE_SCREEN: bool = false;
const RAW_MODE: bool = false;

#[derive(Debug)]
pub struct Screen {
    width:  u16,
    height: u16,
}

impl Screen {
    pub fn new() -> Self {
        let (width, height) = size().expect("Cannot get terminal size");

        Self::enter();
        Self { width, height }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bounds(&self) -> Position {
        (self.width as i32, self.height as i32).into()
    }

    pub fn enter() {
        let out = &mut stdout();

        if RAW_MODE {
            enable_raw_mode().expect("Cannot enable raw mode");
        }

        queue!(out, Hide).expect("Cannot hide cursor");

        if ALTERNATE_SCREEN {
            queue!(out, EnterAlternateScreen).expect("Cannot enter alternate screen");
        }

        out.flush().expect("Cannot flush");
    }

    pub fn leave() {
        let out = &mut stdout();

        if ALTERNATE_SCREEN {
            queue!(out, LeaveAlternateScreen).expect("Cannot leave alternate screen");
        }

        queue!(out, Show).expect("Cannot show cursor");

        out.flush().expect("Cannot flush");

        if RAW_MODE {
            disable_raw_mode().expect("Cannot disable raw mode");
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        Screen::leave();
    }
}
