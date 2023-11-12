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
    let command: Option<Command> = get_command(command);
    if command.is_none() {
        return Err(Error::new(ErrorKind::Other, "command not valid"));
    }

    let output: Vec<u8> = command.unwrap().output()?.stdout;
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
    let command: Option<Command> = get_command(command);
    if command.is_none() {
        return Err(Error::new(ErrorKind::Other, "command not valid"));
    }
    let mut command: Command = command.unwrap();
    let command: &mut Command = if show_output {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit())
    } else {
        command.stdout(Stdio::null()).stderr(Stdio::null())
    };

    command.spawn()?.wait()?;
    Ok(())
}

fn get_command(command: &str) -> Option<Command> {
    // cannot handle redirected output
    if command.contains("|") || command.contains("<") || command.contains(">") {
        return None;
    }

    let mut command_split = command.split_whitespace();

    let program: Option<&str> = command_split.next();
    if program.is_none() {
        return None;
    }

    let mut command: Command = Command::new(program.unwrap());
    loop {
        let arg: Option<&str> = command_split.next();
        if arg.is_none() {
            break;
        }
        command.arg(arg.unwrap());
    }

    Option::from(command)
}
