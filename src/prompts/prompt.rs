use std::io::{self, Error, ErrorKind, Read};

use crate::{ansi, commands};

use super::flush_stdout;

pub fn text(message: &str) -> Result<String, Error> {
    prompt(message, false)
}

pub fn secret(message: &str) -> Result<String, Error> {
    prompt(message, true)
}

fn prompt(message: &str, hide_input: bool) -> Result<String, Error> {
    print!("{}", message);
    flush_stdout()?;

    let mut result: Vec<char> = vec![];
    loop {
        let kp: char = get_keypress_char()?;
        if kp == '\n' {
            break;
        }

        if hide_input {
            ansi::cursor::back();
            print!("*");
            flush_stdout()?;
        }

        result.push(kp);
    }

    Ok(result.into_iter().collect())
}

fn get_keypress_char() -> Result<char, Error> {
    commands::run_silent("stty -F /dev/tty cbreak min 1")?;
    let mut buffer: [u8; 1] = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    commands::run("stty -F /dev/tty sane")?;

    let result: Option<char> = char::from_u32(buffer[0] as u32);
    if result.is_none() {
        return Err(Error::new(ErrorKind::Other, "failed to convert u8 to char"));
    }

    Ok(result.unwrap())
}
