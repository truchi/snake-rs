use crate::{
    events::{poll, Event, KeyCode, KeyEvent},
    KEYS,
    MENU_FPS,
    SNAKE,
    SNAKE_HEIGHT,
    SNAKE_WIDTH,
};
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

#[derive(Debug)]
pub enum MenuAction {
    Level(u8),
    Quit,
}

#[derive(Debug)]
pub struct Menu {
    out:      Stdout,
    levels:   [&'static str; 5],
    selected: u8,
    max:      u8,
    snake_x:  u16,
    snake_y:  u16,
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
            "Level 5 - Snake ... HARDCORE",
        ];
        let levels_width = levels
            .iter()
            .fold(0, |acc, item| acc.max(item.len() as u16))
            + 10; // Adjust...
        let levels_height = levels.len() as u16;
        let keys_width = KEYS.iter().fold(0, |acc, s| acc + s.len() as u16);

        let snake_x = (width - SNAKE_WIDTH) / 2;
        let snake_y = (height - SNAKE_HEIGHT - 2 - levels_height - 2) / 2;
        let levels_x = (width - levels_width) / 2;
        let levels_y = snake_y + SNAKE_HEIGHT + 2;
        let keys_x = (width - keys_width) / 2;
        let keys_y = levels_y + levels_height + 1;

        Self {
            out,
            levels,
            selected,
            max,
            snake_x,
            snake_y,
            levels_x,
            levels_y,
            keys_x,
            keys_y,
        }
    }

    pub fn show(&mut self, snake_y_anim: Option<u16>) -> MenuAction {
        if let Some(mut snake_y_anim) = snake_y_anim {
            loop {
                if snake_y_anim == self.snake_y {
                    break;
                }

                self.snake_frame(&snake_y_anim);
                self.out.flush().unwrap();

                sleep(Duration::from_millis(1000 / MENU_FPS));

                if let Some(Event::Key(KeyEvent { code, .. })) = poll() {
                    if code == KeyCode::Esc {
                        return MenuAction::Quit;
                    }
                }

                self.next_anim_state(&mut snake_y_anim);
                queue!(self.out, Clear(ClearType::All)).unwrap();
            }
        }

        loop {
            self.snake();
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
                            return MenuAction::Level(self.selected);
                        }
                        KeyCode::Esc => {
                            return MenuAction::Quit;
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

    fn snake_frame(&mut self, snake_y_anim: &u16) {
        for y in 0..SNAKE_HEIGHT {
            queue!(
                self.out,
                MoveTo(self.snake_x, snake_y_anim + y),
                Clear(ClearType::CurrentLine),
                Print(&SNAKE[y as usize])
            )
            .unwrap();
        }
    }

    fn next_anim_state(&mut self, snake_y_anim: &mut u16) {
        if *snake_y_anim > self.snake_y {
            *snake_y_anim -= 1;
        } else if *snake_y_anim < self.snake_y {
            *snake_y_anim += 1;
        }
    }

    fn snake(&mut self) {
        for y in 0..SNAKE_HEIGHT {
            queue!(
                self.out,
                MoveTo(self.snake_x, self.snake_y + y),
                Clear(ClearType::CurrentLine),
                Print(&SNAKE[y as usize]),
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
