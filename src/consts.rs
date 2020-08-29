pub const TITLE: &str = "🐍🐍👅";

pub const FPS: u64 = 60;

pub const WELCOME_FPS: u64 = 100;
pub const MENU_FPS: u64 = 20;

const fn millis_to_frames(millis: u64, fps: u64) -> u64 {
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
