use std::io::Error;
use std::process::{Command, ExitStatus, Stdio};
use std::string::FromUtf8Error;

pub struct Operation {
    command: String,
    directory: Option<String>,
    show_output: bool,
}

impl Operation {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            directory: None,
            show_output: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn command<S: Into<String>>(mut self, command: S) -> Self {
        self.command = command.into();
        self
    }

    pub fn directory<S: Into<String>>(mut self, directory: S) -> Self {
        self.directory = Some(directory.into());
        self
    }

    pub fn show_output(mut self, val: bool) -> Self {
        self.show_output = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run(&self) -> Result<(), Error> {
        let mut command: Command = self.get_command()?;

        if self.directory.is_some() {
            command.current_dir(self.directory.clone().unwrap());
        }

        if self.show_output {
            command.stdout(Stdio::inherit()).stderr(Stdio::inherit())
        } else {
            command.stdout(Stdio::null()).stderr(Stdio::null())
        };

        Ok(())
    }

    pub fn run_output(&self) -> Result<String, Error> {
        let mut command: Command = self.get_command()?;

        if self.directory.is_some() {
            command.current_dir(self.directory.clone().unwrap());
        }

        let output: Vec<u8> = command.output()?.stdout;
        let output: Result<String, FromUtf8Error> = String::from_utf8(output);
        if output.is_err() {
            return Err(Error::other("failed to convert command output"));
        }

        Ok(output.unwrap())
    }

    pub fn run_status(&self) -> Result<ExitStatus, Error> {
        let mut command: Command = self.get_command()?;

        if self.directory.is_some() {
            command.current_dir(self.directory.clone().unwrap());
        }

        command.status()
    }

    fn get_command(&self) -> Result<Command, Error> {
        if self.command.contains("|") || self.command.contains("<") || self.command.contains(">") {
            return Err(Error::other("cannot handle redirected output"));
        }

        let mut command_split = self.command.split_whitespace();
        let program: &str = command_split
            .next()
            .ok_or(Error::other("program not provided"))?;
        let mut command: Command = Command::new(program);
        loop {
            let arg: Option<&str> = command_split.next();
            if arg.is_none() {
                break;
            }
            command.arg(arg.unwrap());
        }

        Ok(command)
    }
}
