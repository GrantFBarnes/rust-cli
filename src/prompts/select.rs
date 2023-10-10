use std::io::{self, Read};

use crate::{ansi, commands};

use super::flush_stdout;

enum Motion {
    SUBMIT,
    UP,
    DOWN,
    SELECT,
    EXIT,
    NONE,
}

pub fn select(title: &str, options: Vec<String>, details: Vec<String>) -> Result<String, &str> {
    let indexes: Vec<usize> = get_select_indexes(title, &options, &details, false)?;
    if indexes.len() != 1 {
        return Err("selection invalid");
    }

    let result: Option<&String> = options.get(indexes[0]);
    if result.is_none() {
        return Err("index invalid");
    }

    Ok(result.unwrap().to_string())
}

pub fn mutli_select(
    title: &str,
    options: Vec<String>,
    details: Vec<String>,
) -> Result<Vec<String>, &str> {
    let indexes: Vec<usize> = get_select_indexes(title, &options, &details, true)?;

    let mut result: Vec<String> = vec![];
    for i in indexes {
        let option: Option<&String> = options.get(i);
        if option.is_none() {
            continue;
        }
        result.push(option.unwrap().to_string());
    }

    Ok(result)
}

fn get_select_indexes(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    multi: bool,
) -> Result<Vec<usize>, &'static str> {
    if options.len() == 0 {
        return Err("no options provided");
    }

    if title.len() > 0 {
        ansi::font::bold(true);
        ansi::font::underline(true);
        println!("{}", title);
        ansi::font::reset();
    }

    let mut selected_indexes: Vec<bool> = vec![];
    for _ in 0..options.len() {
        selected_indexes.push(false);
        println!();
    }

    let mut current_index: usize = 0;
    if !multi {
        selected_indexes[0] = true;
    }
    print_options(&options, &details, current_index, &selected_indexes, multi);

    loop {
        let motion: Motion = get_keypress_motion()?;
        match motion {
            Motion::SUBMIT => break,
            Motion::UP => {
                if current_index == 0 {
                    current_index = options.len() - 1;
                } else {
                    current_index -= 1
                }
            }
            Motion::DOWN => {
                current_index += 1;
                if current_index >= options.len() {
                    current_index = 0;
                }
            }
            Motion::SELECT => selected_indexes[current_index] = !selected_indexes[current_index],
            Motion::EXIT => return Err("no selection made"),
            Motion::NONE => continue,
        }

        print_options(&options, &details, current_index, &selected_indexes, multi);
    }

    let mut result: Vec<usize> = vec![];
    if multi {
        for i in 0..selected_indexes.len() {
            if selected_indexes[i] {
                result.push(i);
            }
        }
    } else {
        result.push(current_index);
    }

    Ok(result)
}

fn print_options(
    options: &Vec<String>,
    details: &Vec<String>,
    current_index: usize,
    selected_indexes: &Vec<bool>,
    multi: bool,
) {
    ansi::cursor::previous_lines(options.len());
    for i in 0..options.len() {
        if multi {
            if selected_indexes[i] {
                ansi::font::text_color(ansi::font::Color::GREEN);
                print!(" [X] ");
                ansi::font::reset();
            } else {
                print!(" [ ] ");
            }
        } else {
            if i == current_index {
                print!(" > ");
            } else {
                print!("   ");
            }
        }

        if i == current_index {
            ansi::font::bold(true);
            ansi::font::text_color(ansi::font::Color::CYAN);
        }

        print!("{}", options[i]);

        if details.len() > i {
            if details[i].len() > 0 {
                ansi::font::faint(true);
                print!(" - {}", details[i]);
            }
        }
        ansi::font::reset();
        println!();
    }
}

fn get_keypress_motion() -> Result<Motion, &'static str> {
    commands::run_silent("stty -F /dev/tty cbreak min 1")?;
    let mut buffer: [u8; 3] = [0; 3];
    let keypress: Result<usize, io::Error> = io::stdin().read(&mut buffer);
    commands::run("stty -F /dev/tty sane")?;
    if keypress.is_err() {
        return Err("failed to read keypress");
    }

    ansi::cursor::line_start();
    ansi::erase::line();
    flush_stdout()?;

    match buffer {
        [10, _, _] => {
            ansi::cursor::previous_line();
            flush_stdout()?;
            return Ok(Motion::SUBMIT);
        } // enter
        [27, 0, 0] => return Ok(Motion::EXIT),   // escape
        [113, _, _] => return Ok(Motion::EXIT),  // q
        [27, 91, 65] => return Ok(Motion::UP),   // up arrow
        [27, 91, 66] => return Ok(Motion::DOWN), // down arrow
        [107, _, _] => return Ok(Motion::UP),    // k
        [106, _, _] => return Ok(Motion::DOWN),  // j
        [32, _, _] => return Ok(Motion::SELECT), // space
        _ => return Ok(Motion::NONE),
    }
}
