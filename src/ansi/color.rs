pub enum Color {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::BLACK => "\x1b[30m",
            Color::RED => "\x1b[31m",
            Color::GREEN => "\x1b[32m",
            Color::YELLOW => "\x1b[33m",
            Color::BLUE => "\x1b[34m",
            Color::MAGENTA => "\x1b[35m",
            Color::CYAN => "\x1b[36m",
            Color::WHITE => "\x1b[37m",
        }
    }

    pub fn as_bg_str(&self) -> &'static str {
        match self {
            Color::BLACK => "\x1b[40m",
            Color::RED => "\x1b[41m",
            Color::GREEN => "\x1b[42m",
            Color::YELLOW => "\x1b[44m",
            Color::BLUE => "\x1b[44m",
            Color::MAGENTA => "\x1b[45m",
            Color::CYAN => "\x1b[46m",
            Color::WHITE => "\x1b[47m",
        }
    }
}
