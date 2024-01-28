use std::io::Error;
use std::process::{Command, ExitStatus, Stdio};
use std::string::FromUtf8Error;

pub struct Operation {
    command: String,
    current_dir: Option<String>,
    hide_output: bool,
}

impl Operation {
    pub fn new<S: Into<String>>(command: S) -> Self {
        Self {
            command: command.into(),
            current_dir: None,
            hide_output: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn current_dir<S: Into<String>>(mut self, directory: S) -> Self {
        self.current_dir = Some(directory.into());
        self
    }

    pub fn hide_output(mut self, val: bool) -> Self {
        self.hide_output = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run(&self) -> Result<ExitStatus, Error> {
        let mut command: Command = self.get_command()?;

        if self.hide_output {
            command.stdout(Stdio::null()).stderr(Stdio::null())
        } else {
            command.stdout(Stdio::inherit()).stderr(Stdio::inherit())
        };

        command.status()
    }

    pub fn output(&self) -> Result<String, Error> {
        let mut command: Command = self.get_command()?;

        let output: Vec<u8> = command.output()?.stdout;
        let output: Result<String, FromUtf8Error> = String::from_utf8(output);
        if output.is_err() {
            return Err(Error::other("failed to convert command output"));
        }

        Ok(output.unwrap())
    }

    ////////////////////////////////////////////////////////////////////////////
    /// common run methods

    fn get_command(&self) -> Result<Command, Error> {
        if self.command.contains("&")
            || self.command.contains("|")
            || self.command.contains("<")
            || self.command.contains(">")
        {
            return Err(Error::other("cannot handle complex commands"));
        }

        let mut command_split = self.command.split_whitespace();
        let program: &str = command_split
            .next()
            .ok_or(Error::other("program not provided"))?;
        let mut command: Command = Command::new(program);
        loop {
            if let Some(arg) = command_split.next() {
                command.arg(arg);
            } else {
                break;
            }
        }

        if let Some(dir) = &self.current_dir {
            command.current_dir(dir);
        }

        Ok(command)
    }
}
