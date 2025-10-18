use std::fmt;

#[allow(dead_code)]
enum DebugLevel {
    INFO,
    WARN,
    ERROR,
}

#[macro_export]
macro_rules! debug_print {
    ($l:expr,$($fmt:tt)*) => {
        match $l {
            DebugLevel::INFO => println!("[{}]: {}","i".grey(),format!($($fmt)*)),
            DebugLevel::WARN => eprintln!("[{}]: {}","w".yellow_bold(),format!($($fmt)*)),
            DebugLevel::ERROR => eprintln!("[{}]: {}","e".red_bold(),format!($($fmt)*)),
        }
    };
}

///  type to display terminal attributes
pub struct AttrDisplay {
    attr: String,
    val: String,
}

#[allow(dead_code)]
impl AttrDisplay {
    /// convert AttrDisplay to string
    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.attr, self.val, Self::none())
    }

    /// clear current line
    pub fn clear_line() -> String {
        "\x1b[033[2k\r".to_string()
    }

    /// clear screen and move cursor to top
    pub fn clear_screen() -> String {
        "\x1b[2J\x1b[H".to_string()
    }

    pub fn none() -> String {
        "\x1b[0m".to_string()
    }
}

impl fmt::Display for AttrDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.attr, self.val, Self::none())
    }
}

/// different terminal attributes
#[allow(dead_code)]
pub trait Attrs {
    fn set_attr(attr: &str, v: &str) -> AttrDisplay;

    fn red(&self) -> AttrDisplay;
    fn red_bold(&self) -> AttrDisplay;

    fn green(&self) -> AttrDisplay;
    fn green_bold(&self) -> AttrDisplay;

    fn blue(&self) -> AttrDisplay;
    fn blue_bold(&self) -> AttrDisplay;

    fn yellow(&self) -> AttrDisplay;
    fn yellow_bold(&self) -> AttrDisplay;

    fn grey(&self) -> AttrDisplay;

    fn underline(&self) -> AttrDisplay;
}

impl Attrs for &str {
    fn set_attr(attr: &str, v: &str) -> AttrDisplay {
        AttrDisplay {
            attr: attr.to_string(),
            val: v.to_string(),
        }
    }

    fn red(&self) -> AttrDisplay {
        Self::set_attr("\x1b[0;31m", &self)
    }

    fn red_bold(&self) -> AttrDisplay {
        Self::set_attr("\x1b[1;31m", &self)
    }

    fn green(&self) -> AttrDisplay {
        Self::set_attr("\x1b[0;32m", &self)
    }

    fn green_bold(&self) -> AttrDisplay {
        Self::set_attr("\x1b[1;32m", &self)
    }

    fn blue(&self) -> AttrDisplay {
        Self::set_attr("\x1b[34m", &self)
    }

    fn blue_bold(&self) -> AttrDisplay {
        Self::set_attr("\x1b[1;34m", &self)
    }

    fn yellow(&self) -> AttrDisplay {
        Self::set_attr("\x1b[0;33m", &self)
    }

    fn yellow_bold(&self) -> AttrDisplay {
        Self::set_attr("\x1b[1;33m", &self)
    }

    fn grey(&self) -> AttrDisplay {
        Self::set_attr("\x1b[90m", &self)
    }

    fn underline(&self) -> AttrDisplay {
        Self::set_attr("\x1b[4m", &self)
    }
}
