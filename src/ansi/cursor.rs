// Moves cursor up one row.
pub fn up() {
    print!("\x1b[A");
}

// Moves cursor down one row.
pub fn down() {
    print!("\x1b[B");
}

// Moves cursor forward one column.
pub fn forward() {
    print!("\x1b[C");
}

// Moves cursor back one column.
pub fn back() {
    print!("\x1b[D");
}

// Moves cursor to beginning of the next line.
pub fn next_line() {
    print!("\x1b[E");
}

// Moves cursor to beginning of the next n lines.
pub fn next_lines(n: usize) {
    print!("\x1b[{}E", n);
}

// Moves cursor to beginning of the previous line.
pub fn previous_line() {
    print!("\x1b[F");
}

// Moves cursor to beginning of the previous n lines.
pub fn previous_lines(n: usize) {
    print!("\x1b[{}F", n);
}

// Moves the cursor to start of current line.
pub fn line_start() {
    print!("\x1b[G");
}

// Moves the cursor to row n, column m. The values are 1-based.
pub fn position(n: usize, m: usize) {
    print!("\x1b[{};{}H", n, m);
}
