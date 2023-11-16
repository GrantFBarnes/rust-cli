use std::cmp;
use std::io::{self, Error, ErrorKind, Read};

use crate::{ansi, commands};

use super::flush_stdout;

enum Motion {
    SUBMIT,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    SELECT,
    EXIT,
    NONE,
}

pub fn select_value(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    rows: Option<usize>,
) -> Result<String, Error> {
    let index: usize = select_index(title, options, details, rows)?;
    let result: Option<&String> = options.get(index);
    if result.is_none() {
        return Err(Error::new(ErrorKind::InvalidInput, "index invalid"));
    }

    Ok(result.unwrap().to_string())
}

pub fn select_index(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    rows: Option<usize>,
) -> Result<usize, Error> {
    let indexes: Vec<usize> = get_select_indexes(title, &options, &details, rows, false)?;
    if indexes.len() != 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "selection invalid"));
    }
    Ok(indexes[0])
}

pub fn mutli_select_values(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    rows: Option<usize>,
) -> Result<Vec<String>, Error> {
    let indexes: Vec<usize> = mutli_select_indexes(title, &options, &details, rows)?;

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

pub fn mutli_select_indexes(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    rows: Option<usize>,
) -> Result<Vec<usize>, Error> {
    get_select_indexes(title, &options, &details, rows, true)
}

fn get_select_indexes(
    title: &str,
    options: &Vec<String>,
    details: &Vec<String>,
    rows: Option<usize>,
    multi: bool,
) -> Result<Vec<usize>, Error> {
    if options.len() == 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "no options provided"));
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
    }

    let mut current_index: usize = 0;
    if !multi {
        selected_indexes[0] = true;
    }

    let rows: usize = rows.unwrap_or(10);
    for _ in 0..cmp::min(options.len(), rows) {
        println!();
    }
    let max_page: usize = (options.len() - 1) / rows;
    if max_page > 0 {
        println!();
    }

    print_options(
        &options,
        &details,
        current_index,
        &selected_indexes,
        rows,
        multi,
    );

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
            Motion::LEFT => {
                if options.len() <= rows {
                    continue;
                }

                let current_page: usize = current_index / rows;
                if current_page == 0 {
                    current_index = max_page * rows;
                } else {
                    current_index = (current_page - 1) * rows;
                }
            }
            Motion::RIGHT => {
                if options.len() <= rows {
                    continue;
                }

                let current_page: usize = current_index / rows;
                if current_page == max_page {
                    current_index = 0;
                } else {
                    current_index = (current_page + 1) * rows;
                }
            }
            Motion::SELECT => selected_indexes[current_index] = !selected_indexes[current_index],
            Motion::EXIT => return Err(Error::new(ErrorKind::InvalidInput, "no selection made")),
            Motion::NONE => continue,
        }

        print_options(
            &options,
            &details,
            current_index,
            &selected_indexes,
            rows,
            multi,
        );
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
    rows: usize,
    multi: bool,
) {
    let rows: usize = cmp::min(options.len(), rows);
    for _ in 0..rows {
        ansi::cursor::previous_line();
        ansi::erase::line();
    }
    let max_page: usize = (options.len() - 1) / rows;
    if max_page > 0 {
        ansi::cursor::previous_line();
        ansi::erase::line();
    }

    let skip: usize = (current_index / rows) * rows;
    for i in 0..rows {
        let idx: usize = i + skip;
        if options.len() <= idx {
            println!();
            continue;
        }

        if multi {
            if selected_indexes[idx] {
                ansi::font::text_color(ansi::font::Color::GREEN);
                print!(" [X] ");
                ansi::font::reset();
            } else {
                print!(" [ ] ");
            }
        } else {
            if idx == current_index {
                print!(" > ");
            } else {
                print!("   ");
            }
        }

        if idx == current_index {
            ansi::font::bold(true);
            ansi::font::text_color(ansi::font::Color::CYAN);
        }

        print!("{}", options[idx]);

        if details.len() > idx {
            if details[idx].len() > 0 {
                ansi::font::faint(true);
                print!(" - {}", details[idx]);
            }
        }
        ansi::font::reset();
        println!();
    }

    if max_page > 0 {
        ansi::font::text_color(ansi::font::Color::WHITE);
        ansi::font::faint(true);
        ansi::font::italic(true);
        let current_page: usize = current_index / rows;
        println!("Page [{}/{}]", current_page + 1, max_page + 1);
        ansi::font::reset();
    }
}

fn get_keypress_motion() -> Result<Motion, Error> {
    commands::run_silent("stty -F /dev/tty cbreak min 1")?;
    let mut buffer: [u8; 3] = [0; 3];
    io::stdin().read(&mut buffer)?;
    commands::run("stty -F /dev/tty sane")?;

    ansi::cursor::line_start();
    ansi::erase::line();
    flush_stdout()?;

    match buffer {
        [10, _, _] => {
            ansi::cursor::previous_line();
            flush_stdout()?;
            return Ok(Motion::SUBMIT);
        } // enter
        [27, 0, 0] => return Ok(Motion::EXIT),    // escape
        [113, _, _] => return Ok(Motion::EXIT),   // q
        [27, 91, 65] => return Ok(Motion::UP),    // up arrow
        [27, 91, 66] => return Ok(Motion::DOWN),  // down arrow
        [27, 91, 68] => return Ok(Motion::LEFT),  // left arrow
        [27, 91, 67] => return Ok(Motion::RIGHT), // right arrow
        [108, _, _] => return Ok(Motion::RIGHT),  // l
        [107, _, _] => return Ok(Motion::UP),     // k
        [106, _, _] => return Ok(Motion::DOWN),   // j
        [104, _, _] => return Ok(Motion::LEFT),   // h
        [32, _, _] => return Ok(Motion::SELECT),  // space
        _ => return Ok(Motion::NONE),
    }
}
