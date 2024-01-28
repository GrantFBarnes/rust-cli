use std::io::{self, Error, Read, Write};

use crate::ansi;
use crate::commands;

#[derive(PartialEq)]
pub enum Key {
    LowerA,
    LowerB,
    LowerC,
    LowerD,
    LowerE,
    LowerF,
    LowerG,
    LowerH,
    LowerI,
    LowerJ,
    LowerK,
    LowerL,
    LowerM,
    LowerN,
    LowerO,
    LowerP,
    LowerQ,
    LowerR,
    LowerS,
    LowerT,
    LowerU,
    LowerV,
    LowerW,
    LowerX,
    LowerY,
    LowerZ,

    UpperA,
    UpperB,
    UpperC,
    UpperD,
    UpperE,
    UpperF,
    UpperG,
    UpperH,
    UpperI,
    UpperJ,
    UpperK,
    UpperL,
    UpperM,
    UpperN,
    UpperO,
    UpperP,
    UpperQ,
    UpperR,
    UpperS,
    UpperT,
    UpperU,
    UpperV,
    UpperW,
    UpperX,
    UpperY,
    UpperZ,

    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,

    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,

    Enter,
    Escape,
    Space,
    Backspace,
    Tab,
    ShiftTab,

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
        [97, _, _] => Ok(Key::LowerA),
        [98, _, _] => Ok(Key::LowerB),
        [99, _, _] => Ok(Key::LowerC),
        [100, _, _] => Ok(Key::LowerD),
        [101, _, _] => Ok(Key::LowerE),
        [102, _, _] => Ok(Key::LowerF),
        [103, _, _] => Ok(Key::LowerG),
        [104, _, _] => Ok(Key::LowerH),
        [105, _, _] => Ok(Key::LowerI),
        [106, _, _] => Ok(Key::LowerJ),
        [107, _, _] => Ok(Key::LowerK),
        [108, _, _] => Ok(Key::LowerL),
        [109, _, _] => Ok(Key::LowerM),
        [110, _, _] => Ok(Key::LowerN),
        [111, _, _] => Ok(Key::LowerO),
        [112, _, _] => Ok(Key::LowerP),
        [113, _, _] => Ok(Key::LowerQ),
        [114, _, _] => Ok(Key::LowerR),
        [115, _, _] => Ok(Key::LowerS),
        [116, _, _] => Ok(Key::LowerT),
        [117, _, _] => Ok(Key::LowerU),
        [118, _, _] => Ok(Key::LowerV),
        [119, _, _] => Ok(Key::LowerW),
        [120, _, _] => Ok(Key::LowerX),
        [121, _, _] => Ok(Key::LowerY),
        [122, _, _] => Ok(Key::LowerZ),

        [65, _, _] => Ok(Key::UpperA),
        [66, _, _] => Ok(Key::UpperB),
        [67, _, _] => Ok(Key::UpperC),
        [68, _, _] => Ok(Key::UpperD),
        [69, _, _] => Ok(Key::UpperE),
        [70, _, _] => Ok(Key::UpperF),
        [71, _, _] => Ok(Key::UpperG),
        [72, _, _] => Ok(Key::UpperH),
        [73, _, _] => Ok(Key::UpperI),
        [74, _, _] => Ok(Key::UpperJ),
        [75, _, _] => Ok(Key::UpperK),
        [76, _, _] => Ok(Key::UpperL),
        [77, _, _] => Ok(Key::UpperM),
        [78, _, _] => Ok(Key::UpperN),
        [79, _, _] => Ok(Key::UpperO),
        [80, _, _] => Ok(Key::UpperP),
        [81, _, _] => Ok(Key::UpperQ),
        [82, _, _] => Ok(Key::UpperR),
        [83, _, _] => Ok(Key::UpperS),
        [84, _, _] => Ok(Key::UpperT),
        [85, _, _] => Ok(Key::UpperU),
        [86, _, _] => Ok(Key::UpperV),
        [87, _, _] => Ok(Key::UpperW),
        [88, _, _] => Ok(Key::UpperX),
        [89, _, _] => Ok(Key::UpperY),
        [90, _, _] => Ok(Key::UpperZ),

        [48, _, _] => Ok(Key::Number0),
        [49, _, _] => Ok(Key::Number1),
        [50, _, _] => Ok(Key::Number2),
        [51, _, _] => Ok(Key::Number3),
        [52, _, _] => Ok(Key::Number4),
        [53, _, _] => Ok(Key::Number5),
        [54, _, _] => Ok(Key::Number6),
        [55, _, _] => Ok(Key::Number7),
        [56, _, _] => Ok(Key::Number8),
        [57, _, _] => Ok(Key::Number9),

        [27, 91, 65] => Ok(Key::ArrowUp),
        [27, 91, 66] => Ok(Key::ArrowDown),
        [27, 91, 67] => Ok(Key::ArrowRight),
        [27, 91, 68] => Ok(Key::ArrowLeft),

        [10, 0, 0] => {
            ansi::cursor::previous_line();
            flush_stdout()?;
            Ok(Key::Enter)
        }
        [27, 0, 0] => Ok(Key::Escape),
        [32, 0, 0] => Ok(Key::Space),
        [127, 0, 0] => Ok(Key::Backspace),
        [9, 0, 0] => Ok(Key::Tab),
        [27, 91, 90] => Ok(Key::ShiftTab),

        _ => Ok(Key::Unknown),
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
