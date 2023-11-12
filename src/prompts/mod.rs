use std::io::{self, Error, ErrorKind, Write};

pub mod prompt;
pub mod select;

fn flush_stdout() -> Result<(), Error> {
    if io::stdout().flush().is_err() {
        return Err(Error::new(ErrorKind::Other, "stdout flush failed"));
    }
    Ok(())
}
