use std::io::Error;

use crate::ansi;
use crate::keys;

pub struct Text {
    message: String,
    confirm: bool,
    secret: bool,
    required: bool,
}

impl Text {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            confirm: false,
            secret: false,
            required: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

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

    pub fn run(&self) -> Result<String, Error> {
        print!("{}", self.message);
        keys::flush_stdout()?;
        let input: String = keys::get_line()?;
        if self.secret {
            ansi::cursor::previous_line();
            ansi::erase::line();
            keys::flush_stdout()?;
        }

        if self.confirm {
            print!("Again:");
            keys::flush_stdout()?;
            let confirm: String = keys::get_line()?;
            if self.secret {
                ansi::cursor::previous_line();
                ansi::erase::line();
                keys::flush_stdout()?;
            }

            if input != confirm {
                return Err(Error::other("confirmation doesn't match"));
            }
        }

        if self.required && input.len() == 0 {
            return Err(Error::other("input is required"));
        }

        Ok(input)
    }
}
