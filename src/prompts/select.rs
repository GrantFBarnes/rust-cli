use std::cmp;
use std::io::Error;

use crate::ansi;
use crate::keys;

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

    pub fn option<S: Into<String>>(mut self, option: S) -> Self {
        self.options.push(option.into());
        self.rows_per_page()
    }

    pub fn details<T: ToString>(mut self, details: &[T]) -> Self {
        for detail in details {
            self.details.push(detail.to_string());
        }
        self
    }

    pub fn detail<S: Into<String>>(mut self, detail: S) -> Self {
        self.details.push(detail.into());
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
        self.last_page_index = 0;
        if self.rows_per_page > 0 {
            self.last_page_index = (self.options.len() - 1) / self.rows_per_page;
        }
        self
    }

    ////////////////////////////////////////////////////////////////////////////
    /// run methods

    pub fn run_select(&self) -> Result<Option<(usize, String)>, Error> {
        let indexes: Vec<usize> = self.prompt_and_erase(false)?;

        if indexes.len() > 1 {
            return Err(Error::other("selection invalid"));
        }

        if indexes.len() == 0 {
            return Ok(None);
        }

        let index: usize = indexes[0];
        if let Some(value) = self.options.get(index) {
            return Ok(Some((index, value.to_string())));
        }
        return Err(Error::other("index invalid"));
    }

    pub fn run_multi_select(&self) -> Result<Vec<(usize, String)>, Error> {
        let indexes: Vec<usize> = self.prompt_and_erase(true)?;

        let mut result: Vec<(usize, String)> = vec![];
        for index in indexes {
            if let Some(value) = self.options.get(index) {
                result.push((index, value.to_string()));
            }
        }

        Ok(result)
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
                ansi::erase::line();
                ansi::cursor::previous_line();
                ansi::erase::line();
            }
            keys::flush_stdout()?;
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

fn get_keypress_action() -> Result<Action, Error> {
    match keys::get_keypress()? {
        keys::Key::ArrowDown => Ok(Action::Down),
        keys::Key::ArrowLeft => Ok(Action::Left),
        keys::Key::ArrowRight => Ok(Action::Right),
        keys::Key::ArrowUp => Ok(Action::Up),

        keys::Key::LowerA => Ok(Action::SelectAll),
        keys::Key::LowerH => Ok(Action::Left),
        keys::Key::LowerJ => Ok(Action::Down),
        keys::Key::LowerK => Ok(Action::Up),
        keys::Key::LowerL => Ok(Action::Right),
        keys::Key::LowerQ => Ok(Action::Exit),

        keys::Key::Enter => {
            ansi::cursor::previous_line();
            keys::flush_stdout()?;
            return Ok(Action::Submit);
        }
        keys::Key::Escape => Ok(Action::Exit),
        keys::Key::Space => Ok(Action::Select),
        keys::Key::Tab => Ok(Action::Down),
        keys::Key::ShiftTab => Ok(Action::Up),

        _ => Ok(Action::None),
    }
}
