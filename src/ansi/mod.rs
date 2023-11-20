pub mod cursor;
pub mod erase;
pub mod font;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Black => "\x1b[30m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
        }
    }

    pub fn as_bg_str(&self) -> &str {
        match self {
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[44m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
        }
    }
}

pub enum Cursor {
    Up,
    Down,
    Forward,
    Back,
    NextLine,
    PrevLine,
    LineStart,
}

impl Cursor {
    pub fn as_str(&self) -> &str {
        match self {
            Cursor::Up => "\x1b[A",
            Cursor::Down => "\x1b[B",
            Cursor::Forward => "\x1b[C",
            Cursor::Back => "\x1b[D",
            Cursor::NextLine => "\x1b[E",
            Cursor::PrevLine => "\x1b[F",
            Cursor::LineStart => "\x1b[G",
        }
    }
}

pub enum Erase {
    DisplayToEnd,
    DisplayToBegining,
    Display,
    LineToEnd,
    LineToBegining,
    Line,
}

impl Erase {
    pub fn as_str(&self) -> &str {
        match self {
            Erase::DisplayToEnd => "\x1b[J",
            Erase::DisplayToBegining => "\x1b[1J",
            Erase::Display => "\x1b[2J",
            Erase::LineToEnd => "\x1b[K",
            Erase::LineToBegining => "\x1b[1K",
            Erase::Line => "\x1b[2K",
        }
    }
}

pub enum Font {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    InverColor,
    Hide,
    Strike,
    Default,
}

impl Font {
    pub fn as_str(&self) -> &str {
        match self {
            Font::Reset => "\x1b[0m",
            Font::Bold => "\x1b[1m",
            Font::Faint => "\x1b[2m",
            Font::Italic => "\x1b[3m",
            Font::Underline => "\x1b[4m",
            Font::SlowBlink => "\x1b[5m",
            Font::RapidBlink => "\x1b[6m",
            Font::InverColor => "\x1b[7m",
            Font::Hide => "\x1b[8m",
            Font::Strike => "\x1b[9m",
            Font::Default => "\x1b[10m",
        }
    }

    pub fn as_disable_str(&self) -> &str {
        match self {
            Font::Reset | Font::Default => "",
            Font::Bold | Font::Faint => "\x1b[22m",
            Font::Italic => "\x1b[23m",
            Font::Underline => "\x1b[24m",
            Font::SlowBlink | Font::RapidBlink => "\x1b[25m",
            Font::InverColor => "\x1b[27m",
            Font::Hide => "\x1b[28m",
            Font::Strike => "\x1b[29m",
        }
    }
}
