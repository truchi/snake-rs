use crate::{Menu, Position, Welcome};
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
    },
};
use std::io::{stdout, Stdout, Write};

const ALTERNATE_SCREEN: bool = true;
const RAW_MODE: bool = true;

#[derive(Debug)]
pub struct Screen {
    out:     Stdout,
    entered: bool,
    width:   u16,
    height:  u16,
}

impl Screen {
    pub fn new() -> Self {
        let entered = false;
        let out = stdout();
        let (width, height) = size().expect("Cannot get terminal size");

        Self {
            out,
            entered,
            width,
            height,
        }
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

    pub fn clear(&mut self) {
        execute!(self.out, Clear(ClearType::All)).unwrap();
    }

    pub fn welcome(&mut self) {
        self.clear();
        Welcome::new(self.width, self.height).show();
        self.clear();
    }

    pub fn menu(&mut self) {
        self.clear();
        Menu::new(self.width, self.height).show();
        self.clear();
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
