use crate::events::{poll, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Stdout, Write},
    thread::sleep,
    time::Duration,
};

const HEADER: [&str; 5] = [
    r"🐍🐍👅  🐍    👅      🐍      🐍  👅  🐍🐍👅",
    r"🐍      🐍🐍  🐍     🐍🐍     🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍 🐍 🐍    🐍  🐍    🐍🐍    🐍🐍  ",
    r"    🐍  🐍  🐍🐍   🐍 🐍 🐍   🐍 🐍   🐍    ",
    r"🐍🐍🐍  🐍    🐍  🐍      👅  🐍  🐍  🐍🐍🐍",
];
const HEADER_WIDTH: u16 = HEADER[0].len() as u16 - 19; // Adjust because weird chars
const HEADER_HEIGHT: u16 = HEADER.len() as u16;

const KEYS: [&str; 9] = [
    "[", "↲", " play ", "↑", " prev ", "↓", " next ", "ESC", " quit]",
];

pub enum Selection {
    Level(u8),
    Quit,
}

pub struct Menu {
    out:      Stdout,
    levels:   [&'static str; 5],
    selected: u8,
    max:      u8,
    header_x: u16,
    header_y: u16,
    levels_x: u16,
    levels_y: u16,
    keys_x:   u16,
    keys_y:   u16,
}

impl Menu {
    pub fn new(width: u16, height: u16) -> Self {
        let out = stdout();
        let selected = 0;
        let max = 3;
        let levels = [
            "Level 1 - Snake Pit",
            "Level 2 - Snake Den",
            "Level 3 - Lol",
            "Level 4 - Snake ...",
            "Level 5 - Snake ... MEGA HARDCORE",
        ];
        let levels_width = levels
            .iter()
            .fold(0, |acc, item| acc.max(item.len() as u16))
            + 10; // Adjust...
        let levels_height = levels.len() as u16;
        let keys_width = KEYS.iter().fold(0, |acc, s| acc + s.len() as u16);

        let header_x = (width - HEADER_WIDTH) / 2;
        let header_y = 1;
        let levels_x = (width - levels_width) / 2;
        let levels_y = header_y + HEADER_HEIGHT + 2;
        let keys_x = (width - keys_width) / 2;
        let keys_y = levels_y + levels_height + 1;

        Self {
            out,
            levels,
            selected,
            max,
            header_x,
            header_y,
            levels_x,
            levels_y,
            keys_x,
            keys_y,
        }
    }

    pub fn show(&mut self) -> Selection {
        loop {
            self.header();
            self.levels();
            self.keys();
            self.out.flush().unwrap();

            loop {
                if let Some(Event::Key(KeyEvent { code, .. })) = poll() {
                    match code {
                        KeyCode::Up => {
                            self.prev();
                            break;
                        }
                        KeyCode::Down => {
                            self.next();
                            break;
                        }
                        KeyCode::Enter => {
                            return Selection::Level(self.selected);
                        }
                        KeyCode::Esc => {
                            return Selection::Quit;
                        }
                        _ => {}
                    }
                }

                sleep(Duration::from_millis(100));
            }
        }
    }

    fn prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.max;
        } else {
            self.selected -= 1;
        }
    }

    fn next(&mut self) {
        self.selected += 1;
        if self.selected > self.max {
            self.selected = 0;
        }
    }

    fn header(&mut self) {
        for y in 0..HEADER_HEIGHT {
            queue!(
                self.out,
                MoveTo(self.header_x, self.header_y + y),
                Clear(ClearType::CurrentLine),
                Print(&HEADER[y as usize]),
            )
            .unwrap();
        }
    }

    fn levels(&mut self) {
        for (i, item) in self.levels.clone().iter().enumerate() {
            let (before, attr) = if i as u8 == self.selected {
                ("👅 ", Attribute::Bold)
            } else if i as u8 <= self.max {
                ("🐍 ", Attribute::Reset)
            } else {
                ("   ", Attribute::Dim)
            };

            queue!(
                self.out,
                MoveTo(self.levels_x, self.levels_y + i as u16),
                Clear(ClearType::CurrentLine),
                Print(before),
                SetAttribute(attr),
                Print(item),
                SetAttribute(Attribute::Reset),
            )
            .unwrap();
        }
        ()
    }

    fn keys(&mut self) {
        queue!(
            self.out,
            MoveTo(self.keys_x, self.keys_y),
            Clear(ClearType::CurrentLine),
            Print(KEYS[0]),
            SetAttribute(Attribute::Bold),
            Print(KEYS[1]),
            SetAttribute(Attribute::Reset),
            Print(KEYS[2]),
            SetAttribute(Attribute::Bold),
            Print(KEYS[3]),
            SetAttribute(Attribute::Reset),
            Print(KEYS[4]),
            SetAttribute(Attribute::Bold),
            Print(KEYS[5]),
            SetAttribute(Attribute::Reset),
            Print(KEYS[6]),
            SetAttribute(Attribute::Bold),
            Print(KEYS[7]),
            SetAttribute(Attribute::Reset),
            Print(KEYS[8]),
        )
        .unwrap();
    }
}
