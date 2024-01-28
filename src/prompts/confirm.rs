use std::io::Error;

use crate::keys;

pub struct Confirm {
    message: String,
    default_no: bool,
}

impl Confirm {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            default_no: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn default_no(mut self, val: bool) -> Self {
        self.default_no = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run(&self) -> Result<bool, Error> {
        print!("{}", self.message);
        if self.default_no {
            print!("[y/N]");
        } else {
            print!("[Y/n]");
        }
        keys::flush_stdout()?;

        return match keys::get_line()?.to_lowercase().as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            "" => {
                if self.default_no {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            _ => Err(Error::other("input not valid")),
        };
    }
}
