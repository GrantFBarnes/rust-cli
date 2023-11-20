use crate::ansi::Cursor;

// Moves cursor up one row.
pub fn up() {
    print!("{}", Cursor::Up.as_str());
}

// Moves cursor down one row.
pub fn down() {
    print!("{}", Cursor::Down.as_str());
}

// Moves cursor forward one column.
pub fn forward() {
    print!("{}", Cursor::Forward.as_str());
}

// Moves cursor back one column.
pub fn back() {
    print!("{}", Cursor::Back.as_str());
}

// Moves cursor to beginning of the next line.
pub fn next_line() {
    print!("{}", Cursor::NextLine.as_str());
}

// Moves cursor to beginning of the next n lines.
pub fn next_lines(n: usize) {
    print!("\x1b[{}E", n);
}

// Moves cursor to beginning of the previous line.
pub fn previous_line() {
    print!("{}", Cursor::PrevLine.as_str());
}

// Moves cursor to beginning of the previous n lines.
pub fn previous_lines(n: usize) {
    print!("\x1b[{}F", n);
}

// Moves the cursor to start of current line.
pub fn line_start() {
    print!("{}", Cursor::LineStart.as_str());
}

// Moves the cursor to row n, column m. The values are 1-based.
pub fn position(n: usize, m: usize) {
    print!("\x1b[{};{}H", n, m);
}
