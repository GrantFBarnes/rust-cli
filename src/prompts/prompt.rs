use std::io::{self, Error, Read};

use crate::{ansi, commands};

use super::flush_stdout;

pub struct Prompt {
    message: String,
    confirm: bool,
    secret: bool,
    required: bool,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            confirm: false,
            secret: false,
            required: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    pub fn confirm(mut self, val: bool) -> Self {
        self.confirm = val;
        self
    }

    pub fn secret(mut self, val: bool) -> Self {
        self.secret = val;
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.required = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn prompt(&self) -> Result<String, Error> {
        print!("{}", self.message);
        flush_stdout()?;
        let input: String = self.collect_input()?;

        if self.confirm {
            print!("Again:");
            flush_stdout()?;
            let confirm: String = self.collect_input()?;

            if input != confirm {
                return Err(Error::other("confirmation doesn't match"));
            }
        }

        if self.required && input.len() == 0 {
            return Err(Error::other("input is required"));
        }

        Ok(input)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// common run methods

    fn collect_input(&self) -> Result<String, Error> {
        let mut result: Vec<char> = vec![];
        loop {
            let kp: char = get_keypress_char()?;
            if kp == '\n' {
                break;
            }

            if self.secret {
                ansi::cursor::back();
                print!("*");
                flush_stdout()?;
            }

            result.push(kp);
        }
        Ok(result.into_iter().collect())
    }
}

fn get_keypress_char() -> Result<char, Error> {
    commands::run_silent("stty -F /dev/tty cbreak min 1")?;
    let mut buffer: [u8; 1] = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    commands::run("stty -F /dev/tty sane")?;

    let result: Option<char> = char::from_u32(buffer[0] as u32);
    if result.is_none() {
        return Err(Error::other("failed to convert u8 to char"));
    }

    Ok(result.unwrap())
}
