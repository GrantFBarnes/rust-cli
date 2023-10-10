// Reset font to normal. All attributes turned off.
pub fn reset() {
    print!("\x1b[0m");
}

// Set font bold attribute.
pub fn bold(enable: bool) {
    if enable {
        print!("\x1b[1m");
    } else {
        print!("\x1b[22m");
    }
}

// Set font faint attribute.
pub fn faint(enable: bool) {
    if enable {
        print!("\x1b[2m");
    } else {
        print!("\x1b[22m");
    }
}

// Set font italic attribute.
pub fn italic(enable: bool) {
    if enable {
        print!("\x1b[3m");
    } else {
        print!("\x1b[23m");
    }
}

// Set font underline attribute.
pub fn underline(enable: bool) {
    if enable {
        print!("\x1b[4m");
    } else {
        print!("\x1b[24m");
    }
}

// Set font blink slowly attribute.
pub fn slow_blink(enable: bool) {
    if enable {
        print!("\x1b[5m");
    } else {
        print!("\x1b[25m");
    }
}

// Set font blink rapidly attribute.
pub fn rapid_blink(enable: bool) {
    if enable {
        print!("\x1b[6m");
    } else {
        print!("\x1b[25m");
    }
}

// Set font invert color attribute. (swap foreground and background colors);
pub fn invert_color(enable: bool) {
    if enable {
        print!("\x1b[7m");
    } else {
        print!("\x1b[27m");
    }
}

// Set font hide attribute.
pub fn hide(enable: bool) {
    if enable {
        print!("\x1b[8m");
    } else {
        print!("\x1b[28m");
    }
}

// Set font crossed-out/sriked attribute.
pub fn strike(enable: bool) {
    if enable {
        print!("\x1b[9m");
    } else {
        print!("\x1b[29m");
    }
}

// Set font to primary (default) font.
pub fn default() {
    print!("\x1b[10m");
}

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

// Set font text color.
pub fn text_color(color: Color) {
    match color {
        Color::BLACK => print!("\x1b[30m"),
        Color::RED => print!("\x1b[31m"),
        Color::GREEN => print!("\x1b[32m"),
        Color::YELLOW => print!("\x1b[33m"),
        Color::BLUE => print!("\x1b[34m"),
        Color::MAGENTA => print!("\x1b[35m"),
        Color::CYAN => print!("\x1b[36m"),
        Color::WHITE => print!("\x1b[37m"),
    }
}

// Set font background color.
pub fn background_color(color: Color) {
    match color {
        Color::BLACK => print!("\x1b[40m"),
        Color::RED => print!("\x1b[41m"),
        Color::GREEN => print!("\x1b[42m"),
        Color::YELLOW => print!("\x1b[44m"),
        Color::BLUE => print!("\x1b[44m"),
        Color::MAGENTA => print!("\x1b[45m"),
        Color::CYAN => print!("\x1b[46m"),
        Color::WHITE => print!("\x1b[47m"),
    }
}
