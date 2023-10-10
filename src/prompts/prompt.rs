use std::io::{self, Read};

use crate::{ansi, commands};

use super::flush_stdout;

pub fn text(message: &str) -> Result<String, &str> {
    prompt(message, false)
}

pub fn secret(message: &str) -> Result<String, &str> {
    prompt(message, true)
}

fn prompt(message: &str, hide_input: bool) -> Result<String, &str> {
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

fn get_keypress_char() -> Result<char, &'static str> {
    commands::run_silent("stty -F /dev/tty cbreak min 1")?;
    let mut buffer: [u8; 1] = [0; 1];
    let keypress: Result<(), io::Error> = io::stdin().read_exact(&mut buffer);
    commands::run("stty -F /dev/tty sane")?;
    if keypress.is_err() {
        return Err("failed to read char");
    }

    let result: Option<char> = char::from_u32(buffer[0] as u32);
    if result.is_none() {
        return Err("failed to convert u8 to char");
    }

    Ok(result.unwrap())
}
