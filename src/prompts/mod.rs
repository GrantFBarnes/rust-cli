use std::io::{self, Error, Write};

pub mod prompt;
pub mod select;

fn flush_stdout() -> Result<(), Error> {
    if io::stdout().flush().is_err() {
        return Err(Error::other("stdout flush failed"));
    }
    Ok(())
}
