use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;

pub fn run(command: &str) -> Result<(), Error> {
    run_command(command, true)
}

pub fn run_silent(command: &str) -> Result<(), Error> {
    run_command(command, false)
}

pub fn output(command: &str) -> Result<String, Error> {
    let output: Vec<u8> = get_command(command)?.output()?.stdout;
    let output: Result<String, FromUtf8Error> = String::from_utf8(output);
    if output.is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            "failed to convert command output",
        ));
    }

    Ok(output.unwrap())
}

fn run_command(command: &str, show_output: bool) -> Result<(), Error> {
    if show_output {
        get_command(command)?
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
    } else {
        get_command(command)?
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
    };
    Ok(())
}

fn get_command(command: &str) -> Result<Command, Error> {
    if command.contains("|") || command.contains("<") || command.contains(">") {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "cannot handle redirected output",
        ));
    }

    let mut command_split = command.split_whitespace();
    let program: &str = command_split
        .next()
        .ok_or(Error::new(ErrorKind::InvalidInput, "program not provided"))?;
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
