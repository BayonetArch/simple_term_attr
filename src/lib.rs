use libc::{TIOCGWINSZ, ioctl};
use std::{
    fmt::{self, Debug, Display},
    fs::File,
    io::{self, Write, stdout},
    mem,
    os::fd::AsRawFd,
    process::exit,
};

const ESC: &'static str = "\x1b";

#[allow(dead_code)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

///  type to display terminal attributes
pub struct TerminalAttribute<T: Display> {
    pub attr: String,
    pub val: T,
}

#[repr(C)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

impl<T: Display> StyleAttributes for T {} //everything that implements a Display trait

#[allow(dead_code)]
impl<T: Display> TerminalAttribute<T> {
    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.attr, self.val, Self::reset_attr())
    }

    pub fn reset_attr() -> String {
        format!("{ESC}[0m")
    }
}

impl<T: Display> fmt::Display for TerminalAttribute<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.attr, self.val, Self::reset_attr())
    }
}

/// different terminal attributes
#[allow(dead_code)]
pub trait StyleAttributes {
    fn set_attr<T>(attr: &str, v: T) -> TerminalAttribute<T>
    where
        T: Display,
    {
        TerminalAttribute {
            attr: attr.to_string(),
            val: v,
        }
    }

    fn red(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[0;31m");
        Self::set_attr(&attr, &self)
    }

    fn red_bold(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[1;31m");
        Self::set_attr(&attr, &self)
    }

    fn green(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[0;32m");
        Self::set_attr(&attr, &self)
    }

    fn green_bold(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[1;32m");
        Self::set_attr(&attr, &self)
    }

    fn blue(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[34m");
        Self::set_attr(&attr, &self)
    }

    fn blue_bold(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[1;34m");
        Self::set_attr(&attr, &self)
    }

    fn yellow(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[0;33m");
        Self::set_attr(&attr, &self)
    }

    fn yellow_bold(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[1;33m");
        Self::set_attr(&attr, &self)
    }

    fn grey(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[90m");
        Self::set_attr(&attr, &self)
    }

    fn underline(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[4m");
        Self::set_attr(&attr, &self)
    }
}

#[macro_export]
macro_rules! debug_println {
    ($l:expr,$($fmt:tt)*) => {

        match $l {

            $crate::LogLevel::INFO => println!("[{}] {}",$crate::StyleAttributes::grey(&"i"),format!($($fmt)*)),

            $crate::LogLevel::WARN => eprintln!("[{}] {}",$crate::StyleAttributes::yellow_bold(&"w"),format!($($fmt)*)),

            $crate::LogLevel::ERROR => eprintln!("[{}] {}",$crate::StyleAttributes::red_bold(&"e"),format!($($fmt)*)),

        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($l:expr,$($fmt:tt)*) => {

        match $l {

            $crate::LogLevel::INFO => print!("[{}] {}",$crate::StyleAttributes::grey(&"i"),format!($($fmt)*)),

            $crate::LogLevel::WARN => eprint!("[{}] {}",$crate::StyleAttributes::yellow_bold(&"w"),format!($($fmt)*)),

            $crate::LogLevel::ERROR => eprint!("[{}] {}",$crate::StyleAttributes::red_bold(&"e"),format!($($fmt)*)),

        }
    };
}

/// clear current line
pub fn clear_line() -> io::Result<()> {
    print!("{ESC}[2K\r");
    stdout().flush()
}

/// clear screen and move cursor to top
pub fn clear_screen() -> io::Result<()> {
    print!("{ESC}[2J{ESC}[H");
    stdout().flush()
}

/// move cursor to row,col;
pub fn move_cursor(row: u16, col: u16) -> io::Result<()> {
    print!("{ESC}[{};{}H", row, col);
    stdout().flush()
}

/// move cursor x line up
pub fn move_cursor_up(row: u16) -> io::Result<()> {
    print!("{ESC}[{row}A");
    stdout().flush()
}

/// move cursor x line down
pub fn move_cursor_down(row: u16) -> io::Result<()> {
    print!("{ESC}[{row}B");
    stdout().flush()
}

/// save current cursor position
pub fn save_cursor_pos() -> io::Result<()> {
    print!("{ESC}[s");
    stdout().flush()
}

/// restore  cursor position
pub fn restore_cursor_pos() -> io::Result<()> {
    print!("{ESC}[u");
    stdout().flush()
}

/// hide the terminal cursor
pub fn hide_cursor() -> io::Result<()> {
    print!("{ESC}[?25l");
    stdout().flush()
}
/// show the terminal cursor
pub fn show_cursor() -> io::Result<()> {
    print!("{ESC}[?25h");
    stdout().flush()
}

// get the current terminal size.returns a tuple of ws_row and ws_col
pub fn get_terminal_size() -> (u16, u16) {
    let stdout = File::open("/dev/tty").unwrap();
    let fd = stdout.as_raw_fd();
    let mut ws: Winsize = unsafe { mem::zeroed() };

    let res = unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) };
    if res != 0 {
        debug_print!(LogLevel::ERROR, "ioctl syscall failed");
        exit(1);
    }

    (ws.ws_col, ws.ws_row)
}

pub fn set_scrollable_region(col: u16, row: u16) -> io::Result<()> {
    print!("\x1b[{};{}r", col, row);
    stdout().flush()
}
