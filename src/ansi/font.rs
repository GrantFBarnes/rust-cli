use super::color::Color;

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

// Set font text color.
pub fn text_color(color: Color) {
    print!("{}", color.as_str());
}

// Set font background color.
pub fn background_color(color: Color) {
    print!("{}", color.as_bg_str());
}
