use libc::{TCSANOW, cfmakeraw, tcgetattr, tcsetattr, termios};
use std::io::stdin;
use std::mem;
use std::os::unix::io::AsRawFd;

static mut ORIG_TERMIOS: Option<termios> = None;

pub fn enable_raw_mode() {
    unsafe {
        let fd = stdin().as_raw_fd();
        let mut term: termios = mem::zeroed();
        tcgetattr(fd, &mut term);
        ORIG_TERMIOS = Some(term);
        let mut raw = term;
        cfmakeraw(&mut raw);
        tcsetattr(fd, TCSANOW, &raw);
    }
}

pub fn disable_raw_mode() {
    unsafe {
        if let Some(orig) = ORIG_TERMIOS {
            let fd = stdin().as_raw_fd();
            tcsetattr(fd, TCSANOW, &orig);
        }
    }
}

pub const CLEAR: &str = "\x1b[2J";
pub const LN: &str = "\r\n";
pub const RESET: &str = "\x1b[0m";

pub fn set_cursor(row: u16, col: u16) -> String {
    format!("\x1b[{row};{col}H")
}

pub fn fg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

pub fn bg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

pub struct RawMode;

impl RawMode {
    pub fn new() -> Self {
        enable_raw_mode();
        RawMode
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}
