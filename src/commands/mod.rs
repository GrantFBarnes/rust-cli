use std::io::Error;
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::string::FromUtf8Error;

pub fn run(command: &str) -> Result<(), &str> {
    run_command(command, true)
}

pub fn run_silent(command: &str) -> Result<(), &str> {
    run_command(command, false)
}

pub fn output(command: &str) -> Result<String, &str> {
    let command: Option<Command> = get_command(command);
    if command.is_none() {
        return Err("command not valid");
    }

    let output: Result<Output, Error> = command.unwrap().output();
    if output.is_err() {
        return Err("failed to get command output");
    }
    let output: Output = output.unwrap();
    let output: Vec<u8> = output.stdout;
    let output: Result<String, FromUtf8Error> = String::from_utf8(output);
    if output.is_err() {
        return Err("failed to convert command output");
    }

    Ok(output.unwrap())
}

fn run_command(command: &str, show_output: bool) -> Result<(), &str> {
    let command: Option<Command> = get_command(command);
    if command.is_none() {
        return Err("command not valid");
    }
    let mut command: Command = command.unwrap();
    let command: &mut Command = if show_output {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit())
    } else {
        command.stdout(Stdio::null()).stderr(Stdio::null())
    };

    let result: Result<Child, Error> = command.spawn();
    if result.is_err() {
        return Err("failed to run command");
    }

    let result: Result<ExitStatus, Error> = result.unwrap().wait();
    if result.is_err() {
        return Err("failed to wait for command");
    }

    Ok(())
}

fn get_command(command: &str) -> Option<Command> {
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
