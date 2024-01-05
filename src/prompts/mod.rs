use std::cmp;
use std::io::{self, Error, Read, Write};

use crate::{ansi, commands};

pub struct Confirm {
    message: String,
    default_no: bool,
}

impl Confirm {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            default_no: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    pub fn default_no(mut self, val: bool) -> Self {
        self.default_no = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run(&self) -> Result<bool, Error> {
        print!("{}", self.message);
        if self.default_no {
            print!("[y/N]");
        } else {
            print!("[Y/n]");
        }
        flush_stdout()?;

        return match read_line()?.to_lowercase().as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            "" => {
                if self.default_no {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            _ => Err(Error::other("input not valid")),
        };
    }
}

pub struct Text {
    message: String,
    confirm: bool,
    secret: bool,
    required: bool,
}

impl Text {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            confirm: false,
            secret: false,
            required: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = message.into();
        self
    }

    pub fn confirm(mut self, val: bool) -> Self {
        self.confirm = val;
        self
    }

    pub fn secret(mut self, val: bool) -> Self {
        self.secret = val;
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.required = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run(&self) -> Result<String, Error> {
        print!("{}", self.message);
        flush_stdout()?;
        let input: String = read_line()?;
        if self.secret {
            ansi::cursor::previous_line();
            ansi::erase::line();
            flush_stdout()?;
        }

        if self.confirm {
            print!("Again:");
            flush_stdout()?;
            let confirm: String = read_line()?;
            if self.secret {
                ansi::cursor::previous_line();
                ansi::erase::line();
                flush_stdout()?;
            }

            if input != confirm {
                return Err(Error::other("confirmation doesn't match"));
            }
        }

        if self.required && input.len() == 0 {
            return Err(Error::other("input is required"));
        }

        Ok(input)
    }
}

enum Action {
    Submit,
    Up,
    Down,
    Left,
    Right,
    Select,
    SelectAll,
    Exit,
    None,
}

pub struct Select {
    // input parameters
    title: Option<String>,
    options: Vec<String>,
    details: Vec<String>,
    default_index: usize,
    max_rows_per_page: usize,
    erase_after: bool,

    // calculated parameters
    rows_per_page: usize,
    last_page_index: usize,
}

impl Select {
    pub fn new() -> Self {
        Self {
            title: None,
            options: vec![],
            details: vec![],
            default_index: 0,
            max_rows_per_page: 20,
            erase_after: false,

            rows_per_page: 0,
            last_page_index: 0,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    /// input parameter set methods

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn options<T: ToString>(mut self, options: &[T]) -> Self {
        for option in options {
            self.options.push(option.to_string());
        }
        self.rows_per_page()
    }

    pub fn option<T: ToString>(mut self, option: T) -> Self {
        self.options.push(option.to_string());
        self.rows_per_page()
    }

    pub fn details<T: ToString>(mut self, details: &[T]) -> Self {
        for detail in details {
            self.details.push(detail.to_string());
        }
        self
    }

    pub fn detail<T: ToString>(mut self, detail: T) -> Self {
        self.details.push(detail.to_string());
        self
    }

    pub fn default_index(mut self, val: usize) -> Self {
        self.default_index = val;
        self
    }

    pub fn max_rows_per_page(mut self, val: usize) -> Self {
        self.max_rows_per_page = if val < 1 { 1 } else { val };
        self.rows_per_page()
    }

    pub fn erase_after(mut self, val: bool) -> Self {
        self.erase_after = val;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// calculated parameter set methods

    fn rows_per_page(mut self) -> Self {
        self.rows_per_page = cmp::min(self.options.len(), self.max_rows_per_page);
        self.last_page_index()
    }

    fn last_page_index(mut self) -> Self {
        self.last_page_index = (self.options.len() - 1) / self.rows_per_page;
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run_select_value(&self) -> Result<Option<String>, Error> {
        let index: Option<usize> = self.run_select_index()?;
        if index.is_none() {
            return Ok(None);
        }
        let result: Option<&String> = self.options.get(index.unwrap());
        if result.is_none() {
            return Err(Error::other("index invalid"));
        }
        Ok(Some(result.unwrap().to_string()))
    }

    pub fn run_select_index(&self) -> Result<Option<usize>, Error> {
        let indexes: Vec<usize> = self.prompt_and_erase(false)?;
        if indexes.len() > 1 {
            return Err(Error::other("selection invalid"));
        }
        if indexes.len() == 0 {
            return Ok(None);
        }
        Ok(Some(indexes[0]))
    }

    pub fn run_multi_select_values(&self) -> Result<Vec<String>, Error> {
        let indexes: Vec<usize> = self.run_multi_select_indexes()?;

        let mut result: Vec<String> = vec![];
        for i in indexes {
            let option: Option<&String> = self.options.get(i);
            if option.is_none() {
                continue;
            }
            result.push(option.unwrap().to_string());
        }

        Ok(result)
    }

    pub fn run_multi_select_indexes(&self) -> Result<Vec<usize>, Error> {
        self.prompt_and_erase(true)
    }

    ////////////////////////////////////////////////////////////////////////////
    /// common run methods

    fn prompt_and_erase(&self, is_multi_select: bool) -> Result<Vec<usize>, Error> {
        let result = self.prompt(is_multi_select);
        if self.erase_after {
            let mut lines_to_erase: usize = self.rows_per_page;
            if self.title.is_some() {
                lines_to_erase += 1;
            }
            if self.last_page_index > 0 {
                lines_to_erase += 1;
            }
            for _ in 0..lines_to_erase {
                ansi::cursor::previous_line();
                ansi::erase::line();
            }
            flush_stdout()?;
        }
        result
    }

    fn prompt(&self, is_multi_select: bool) -> Result<Vec<usize>, Error> {
        if self.options.len() == 0 {
            return Err(Error::other("no options provided"));
        }

        if self.default_index >= self.options.len() {
            return Err(Error::other("default index out of range"));
        }

        if self.title.is_some() {
            ansi::font::bold(true);
            ansi::font::underline(true);
            println!("{}", self.title.clone().unwrap());
            ansi::font::reset();
        }

        let mut current_index: usize = self.default_index;
        let mut selected_indexes: Vec<bool> = vec![];
        for _ in 0..self.options.len() {
            selected_indexes.push(false);
        }

        for _ in 0..self.rows_per_page {
            println!();
        }
        if self.last_page_index > 0 {
            println!();
        }

        self.print_options(current_index, &selected_indexes, is_multi_select);

        let mut result: Vec<usize> = vec![];
        loop {
            match get_keypress_action()? {
                Action::Exit => return Ok(result),
                Action::Submit => break,
                Action::Up => {
                    if current_index == 0 {
                        current_index = self.options.len() - 1;
                    } else {
                        current_index -= 1
                    }
                }
                Action::Down => {
                    current_index += 1;
                    if current_index >= self.options.len() {
                        current_index = 0;
                    }
                }
                Action::Left => {
                    if self.last_page_index == 0 {
                        continue;
                    }

                    let current_page: usize = current_index / self.rows_per_page;
                    if current_page == 0 {
                        current_index = self.last_page_index * self.rows_per_page;
                    } else {
                        current_index = (current_page - 1) * self.rows_per_page;
                    }
                }
                Action::Right => {
                    if self.last_page_index == 0 {
                        continue;
                    }

                    let current_page: usize = current_index / self.rows_per_page;
                    if current_page == self.last_page_index {
                        current_index = 0;
                    } else {
                        current_index = (current_page + 1) * self.rows_per_page;
                    }
                }
                Action::Select => {
                    if is_multi_select {
                        selected_indexes[current_index] = !selected_indexes[current_index]
                    }
                }
                Action::SelectAll => {
                    if is_multi_select {
                        let all_selected = selected_indexes.iter().all(|&x| x);
                        for i in 0..selected_indexes.len() {
                            selected_indexes[i] = !all_selected;
                        }
                    }
                }
                Action::None => continue,
            }

            self.print_options(current_index, &selected_indexes, is_multi_select);
        }

        if is_multi_select {
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
        &self,
        current_index: usize,
        selected_indexes: &Vec<bool>,
        is_multi_select: bool,
    ) {
        for _ in 0..self.rows_per_page {
            ansi::cursor::previous_line();
            ansi::erase::line();
        }
        if self.last_page_index > 0 {
            ansi::cursor::previous_line();
            ansi::erase::line();
        }

        let skip: usize = (current_index / self.rows_per_page) * self.rows_per_page;
        for i in 0..self.rows_per_page {
            let idx: usize = i + skip;
            if self.options.len() <= idx {
                println!();
                continue;
            }

            if is_multi_select {
                if selected_indexes[idx] {
                    ansi::font::text_color(ansi::Color::Green);
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
                ansi::font::text_color(ansi::Color::Cyan);
            }

            print!("{}", self.options[idx]);

            if self.details.len() > idx {
                if self.details[idx].len() > 0 {
                    ansi::font::faint(true);
                    print!(" - {}", self.details[idx]);
                }
            }
            ansi::font::reset();
            println!();
        }

        if self.last_page_index > 0 {
            ansi::font::text_color(ansi::Color::White);
            ansi::font::faint(true);
            ansi::font::italic(true);
            let current_page: usize = current_index / self.rows_per_page;
            println!("Page [{}/{}]", current_page + 1, self.last_page_index + 1);
            ansi::font::reset();
        }
    }
}

fn get_keypress_action() -> Result<Action, Error> {
    commands::Operation::new()
        .command("stty -F /dev/tty cbreak min 1")
        .run()?;
    let mut buffer: [u8; 3] = [0; 3];
    io::stdin().read(&mut buffer)?;
    commands::Operation::new()
        .command("stty -F /dev/tty sane")
        .run()?;

    ansi::cursor::line_start();
    ansi::erase::line();
    flush_stdout()?;

    match buffer {
        [10, _, _] => {
            ansi::cursor::previous_line();
            flush_stdout()?;
            return Ok(Action::Submit);
        } // enter
        [27, 91, 65] => return Ok(Action::Up),      // up arrow
        [27, 91, 66] => return Ok(Action::Down),    // down arrow
        [27, 91, 68] => return Ok(Action::Left),    // left arrow
        [27, 91, 67] => return Ok(Action::Right),   // right arrow
        [107, _, _] => return Ok(Action::Up),       // k
        [106, _, _] => return Ok(Action::Down),     // j
        [104, _, _] => return Ok(Action::Left),     // h
        [108, _, _] => return Ok(Action::Right),    // l
        [27, 91, 90] => return Ok(Action::Up),      // shift tab
        [9, _, _] => return Ok(Action::Down),       // tab
        [32, _, _] => return Ok(Action::Select),    // space
        [97, _, _] => return Ok(Action::SelectAll), // a
        [27, 0, 0] => return Ok(Action::Exit),      // escape
        [113, _, _] => return Ok(Action::Exit),     // q
        _ => return Ok(Action::None),
    }
}

fn read_line() -> Result<String, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim_end_matches('\n').into())
}

fn flush_stdout() -> Result<(), Error> {
    if io::stdout().flush().is_err() {
        return Err(Error::other("stdout flush failed"));
    }
    Ok(())
}
