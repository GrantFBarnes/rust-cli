use crate::ansi::Font;

use super::Color;

// Reset font to normal. All attributes turned off.
pub fn reset() {
    print!("{}", Font::Reset.as_str());
}

// Set font bold attribute.
pub fn bold(enable: bool) {
    if enable {
        print!("{}", Font::Bold.as_str());
    } else {
        print!("{}", Font::Bold.as_disable_str());
    }
}

// Set font faint attribute.
pub fn faint(enable: bool) {
    if enable {
        print!("{}", Font::Faint.as_str());
    } else {
        print!("{}", Font::Faint.as_disable_str());
    }
}

// Set font italic attribute.
pub fn italic(enable: bool) {
    if enable {
        print!("{}", Font::Italic.as_str());
    } else {
        print!("{}", Font::Italic.as_disable_str());
    }
}

// Set font underline attribute.
pub fn underline(enable: bool) {
    if enable {
        print!("{}", Font::Underline.as_str());
    } else {
        print!("{}", Font::Underline.as_disable_str());
    }
}

// Set font blink slowly attribute.
pub fn slow_blink(enable: bool) {
    if enable {
        print!("{}", Font::SlowBlink.as_str());
    } else {
        print!("{}", Font::SlowBlink.as_disable_str());
    }
}

// Set font blink rapidly attribute.
pub fn rapid_blink(enable: bool) {
    if enable {
        print!("{}", Font::RapidBlink.as_str());
    } else {
        print!("{}", Font::RapidBlink.as_disable_str());
    }
}

// Set font invert color attribute. (swap foreground and background colors);
pub fn invert_color(enable: bool) {
    if enable {
        print!("{}", Font::InverColor.as_str());
    } else {
        print!("{}", Font::InverColor.as_disable_str());
    }
}

// Set font hide attribute.
pub fn hide(enable: bool) {
    if enable {
        print!("{}", Font::Hide.as_str());
    } else {
        print!("{}", Font::Hide.as_disable_str());
    }
}

// Set font crossed-out/sriked attribute.
pub fn strike(enable: bool) {
    if enable {
        print!("{}", Font::Strike.as_str());
    } else {
        print!("{}", Font::Strike.as_disable_str());
    }
}

// Set font to primary (default) font.
pub fn default() {
    print!("{}", Font::Default.as_str());
}

// Set font text color.
pub fn text_color(color: Color) {
    print!("{}", color.as_str());
}

// Set font background color.
pub fn background_color(color: Color) {
    print!("{}", color.as_bg_str());
}
