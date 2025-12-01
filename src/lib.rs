use std::{
    error::Error,
    ffi::{c_int, c_ushort},
    fmt::{self, Display},
    io::{self, Write, stdout},
    mem::zeroed,
};

const ESC: &'static str = "\x1b";

#[allow(dead_code)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

#[cfg(unix)]
mod unix {
    use super::*;
    #[repr(C)]
    struct WinSize {
        ws_row: c_ushort,
        ws_col: c_ushort,
        ws_xpixel: c_ushort,
        ws_ypixel: c_ushort,
    }

    const TIOCGWINSZ: c_int = 21523;
    unsafe extern "C" {
        fn ioctl(fd: c_int, request: c_int, ...) -> c_int;
    }

    pub fn get_terminal_size() -> Result<(u16, u16), Box<dyn Error>> {
        let mut ws: WinSize = unsafe { zeroed() };
        let ret = unsafe { ioctl(0, TIOCGWINSZ, &mut ws) };
        if ret != 0 {
            return Err("Could not get window size. 'ioctl' syscall failed.".into());
        }
        Ok((ws.ws_row, ws.ws_col))
    }
}

#[cfg(windows)]
mod windows {
    use std::io;
    use windows::Win32::System::Console::{
        CONSOLE_SCREEN_BUFFER_INFO, GetConsoleScreenBufferInfo, GetStdHandle, STD_OUTPUT_HANDLE,
    };

    pub fn get_terminal_size() -> io::Result<(u16, u16)> {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if handle.is_invalid() {
                return Err(io::Error::last_os_error());
            }

            let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::default();

            if !GetConsoleScreenBufferInfo(handle, &mut csbi).as_bool() {
                return Err(io::Error::last_os_error());
            }

            let width = (csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16;
            let height = (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16;

            Ok((width, height))
        }
    }
}

/// get the current terminal size.returns a tuple of ws_row and ws_col
pub fn get_terminal_size() -> Result<(u16, u16), Box<dyn Error>> {
    #[cfg(unix)]
    {
        unix::get_terminal_size()
    }
    #[cfg(windows)]
    {
        windows::get_terminal_size()
    }
}

///  type to display terminal attributes
pub struct TerminalAttribute<T: Display> {
    pub attr: String,
    pub val: T,
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

    fn grey_bold(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[1;90m");
        Self::set_attr(&attr, &self)
    }

    fn underline(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[4m");
        Self::set_attr(&attr, &self)
    }

    fn bg_red(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[41m");
        Self::set_attr(&attr, &self)
    }

    fn bg_yellow(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[43m");
        Self::set_attr(&attr, &self)
    }

    fn bg_green(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[42m");
        Self::set_attr(&attr, &self)
    }

    fn bg_cyan(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[46m");
        Self::set_attr(&attr, &self)
    }

    fn bg_grey(&self) -> TerminalAttribute<&Self>
    where
        Self: Display,
    {
        let attr = format!("{ESC}[47m");
        Self::set_attr(&attr, &self)
    }
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

pub fn set_scrollable_region(col: u16, row: u16) -> io::Result<()> {
    print!("\x1b[{};{}r", col, row);
    stdout().flush()
}
