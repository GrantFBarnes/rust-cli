// Erase display from cursor to end.
pub fn display_to_end() {
    print!("\x1b[J");
}

// Erase display from cursor to beginning.
pub fn display_to_beginning() {
    print!("\x1b[1J");
}

// Erase entire display and place cursor at upper left.
pub fn display() {
    print!("\x1b[2J");
}

// Erase line from cursor to end.
pub fn line_to_end() {
    print!("\x1b[K");
}

// Erase line from cursor to beginning.
pub fn line_to_beginning() {
    print!("\x1b[1K");
}

// Erase enire line, cursor position does not change.
pub fn line() {
    print!("\x1b[2K");
}
