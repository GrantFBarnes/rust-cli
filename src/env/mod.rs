use std::env::{self, VarError};

pub fn home() -> Result<String, VarError> {
    env::var("HOME")
}
