use crossterm::event::{poll as xpoll, read};
use std::time::Duration;

pub use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};

pub fn poll() -> Option<Event> {
    if let Ok(true) = xpoll(Duration::from_secs(0)) {
        if let Ok(event) = read() {
            return Some(event);
        }
    }

    None
}

pub fn poll_all() -> Vec<Event> {
    let mut events = vec![];

    while let Some(event) = poll() {
        events.push(event);
    }

    events
}
