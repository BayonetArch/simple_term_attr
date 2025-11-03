use libc::{TIOCGWINSZ, ioctl};
use std::{
    fmt,
    fs::File,
    io::{Write, stdout},
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
pub struct DisplayAttribute {
    attr: String,
    val: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

#[allow(dead_code)]
impl DisplayAttribute {
    /// convert AttrDisplay to string
    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.attr, self.val, Self::none())
    }

    /// clear current line
    pub fn clear_line() {
        print!("{ESC}[2K\r");
    }

    /// clear screen and move cursor to top
    pub fn clear_screen() -> std::io::Result<()> {
        print!("{ESC}[2J{ESC}[H");
        stdout().flush()
    }

    /// move cursor to row,col;
    pub fn move_cursor(row: u16, col: u16) {
        print!("{ESC}[{};{}H", row, col);
    }

    /// move cursor x line up
    pub fn move_cursor_x_lines_up(row: u16) {
        print!("{ESC}[{row}A");
    }

    /// move cursor x line down
    pub fn move_cursor_x_lines_down(row: u16) {
        print!("{ESC}[{row}B");
    }

    /// save current cursor position
    pub fn save_cursor_pos() {
        print!("{ESC}[s");
    }

    /// restore  cursor position
    pub fn restore_cursor_pos() {
        print!("{ESC}[u");
    }

    /// hide the terminal cursor
    pub fn hide_cursor() -> std::io::Result<()> {
        print!("{ESC}[?25l");
        stdout().flush()
    }
    /// show the terminal cursor
    pub fn show_cursor() -> std::io::Result<()> {
        print!("{ESC}[?25h");
        stdout().flush()
    }

    // get the current terminal size.returns a tuple of ws_row and ws_col
    pub fn get_term_size() -> (u16, u16) {
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

    pub fn set_scrollable_region(col: u16, row: u16) {
        print!("\x1b[{};{}r", col, row);
    }

    pub fn none() -> String {
        format!("{ESC}[0m")
    }
}

impl fmt::Display for DisplayAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.attr, self.val, Self::none())
    }
}

/// different terminal attributes
#[allow(dead_code)]
pub trait StyleAttributes {
    fn set_attr(attr: &str, v: &str) -> DisplayAttribute;

    fn red(&self) -> DisplayAttribute;
    fn red_bold(&self) -> DisplayAttribute;

    fn green(&self) -> DisplayAttribute;
    fn green_bold(&self) -> DisplayAttribute;

    fn blue(&self) -> DisplayAttribute;
    fn blue_bold(&self) -> DisplayAttribute;

    fn yellow(&self) -> DisplayAttribute;
    fn yellow_bold(&self) -> DisplayAttribute;

    fn grey(&self) -> DisplayAttribute;

    fn underline(&self) -> DisplayAttribute;
}

impl StyleAttributes for &str {
    fn set_attr(attr: &str, v: &str) -> DisplayAttribute {
        DisplayAttribute {
            attr: attr.to_string(),
            val: v.to_string(),
        }
    }

    fn red(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[0;31m");
        Self::set_attr(&attr, &self)
    }

    fn red_bold(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[1;31m");
        Self::set_attr(&attr, &self)
    }

    fn green(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[0;32m");
        Self::set_attr(&attr, &self)
    }

    fn green_bold(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[1;32m");
        Self::set_attr(&attr, &self)
    }

    fn blue(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[34m");
        Self::set_attr(&attr, &self)
    }

    fn blue_bold(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[1;34m");
        Self::set_attr(&attr, &self)
    }

    fn yellow(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[0;33m");
        Self::set_attr(&attr, &self)
    }

    fn yellow_bold(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[1;33m");
        Self::set_attr(&attr, &self)
    }

    fn grey(&self) -> DisplayAttribute {
        let attr = format!("{ESC}[90m");
        Self::set_attr(&attr, &self)
    }

    fn underline(&self) -> DisplayAttribute {
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
