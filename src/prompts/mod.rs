use std::io::{self, Write};

pub mod prompt;
pub mod select;

fn flush_stdout() -> Result<(), &'static str> {
    if io::stdout().flush().is_err() {
        return Err("stdout flush failed");
    }
    Ok(())
}
