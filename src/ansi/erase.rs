use crate::ansi::Erase;

// Erase display from cursor to end.
pub fn display_to_end() {
    print!("{}", Erase::DisplayToEnd.as_str());
}

// Erase display from cursor to beginning.
pub fn display_to_beginning() {
    print!("{}", Erase::DisplayToBegining.as_str());
}

// Erase entire display and place cursor at upper left.
pub fn display() {
    print!("{}", Erase::Display.as_str());
}

// Erase line from cursor to end.
pub fn line_to_end() {
    print!("{}", Erase::LineToEnd.as_str());
}

// Erase line from cursor to beginning.
pub fn line_to_beginning() {
    print!("{}", Erase::LineToBegining.as_str());
}

// Erase enire line, cursor position does not change.
pub fn line() {
    print!("{}", Erase::Line.as_str());
}
