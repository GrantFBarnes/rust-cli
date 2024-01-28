use std::io::{self, Error, Read, Write};

use crate::ansi;
use crate::commands;

#[derive(PartialEq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

    Enter,
    Escape,
    Backspace,
    Tab,
    ShiftTab,
    Space,

    Unknown,
}

pub fn get_keypress() -> Result<Key, Error> {
    commands::Operation::new("stty -F /dev/tty cbreak min 1").run()?;
    let mut buffer: [u8; 3] = [0; 3];
    io::stdin().read(&mut buffer)?;
    commands::Operation::new("stty -F /dev/tty sane").run()?;

    ansi::cursor::line_start();
    ansi::erase::line();
    flush_stdout()?;

    match buffer {
        [10, _, _] => {
            ansi::cursor::previous_line();
            flush_stdout()?;
            Ok(Key::Enter)
        }
        [27, 91, 65] => Ok(Key::UpArrow),
        [27, 91, 66] => Ok(Key::DownArrow),
        [27, 91, 68] => Ok(Key::LeftArrow),
        [27, 91, 67] => Ok(Key::RightArrow),
        [107, _, _] => Ok(Key::K),
        [106, _, _] => Ok(Key::J),
        [104, _, _] => Ok(Key::H),
        [108, _, _] => Ok(Key::L),
        [27, 91, 90] => Ok(Key::ShiftTab),
        [9, _, _] => Ok(Key::Tab),
        [32, _, _] => Ok(Key::Space),
        [97, _, _] => Ok(Key::A),
        [27, 0, 0] => Ok(Key::Escape),
        [113, _, _] => Ok(Key::Q),
        _ => {
            dbg!(buffer);
            Ok(Key::Unknown)
        }
    }
}

pub fn get_line() -> Result<String, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end_matches('\n').into())
}

pub fn flush_stdout() -> Result<(), Error> {
    if io::stdout().flush().is_err() {
        return Err(Error::other("stdout flush failed"));
    }
    Ok(())
}
