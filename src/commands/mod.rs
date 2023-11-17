use std::io::Error;
use std::process::{Command, Stdio};
use std::string::FromUtf8Error;

pub fn run<S: Into<String>>(command: S) -> Result<(), Error> {
    run_command(command.into(), true)
}

pub fn run_silent<S: Into<String>>(command: S) -> Result<(), Error> {
    run_command(command.into(), false)
}

pub fn output<S: Into<String>>(command: S) -> Result<String, Error> {
    let output: Vec<u8> = get_command(command.into())?.output()?.stdout;
    let output: Result<String, FromUtf8Error> = String::from_utf8(output);
    if output.is_err() {
        return Err(Error::other("failed to convert command output"));
    }

    Ok(output.unwrap())
}

fn run_command(command: String, show_output: bool) -> Result<(), Error> {
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

fn get_command(command: String) -> Result<Command, Error> {
    if command.contains("|") || command.contains("<") || command.contains(">") {
        return Err(Error::other("cannot handle redirected output"));
    }

    let mut command_split = command.split_whitespace();
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
