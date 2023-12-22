use crate::utils::{Position, Size};
use std::io::{stdin, stdout, Error, Stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,      // this -1 is beacause I use tmux and it uses one line at the
                height: size.1 - 1, // end of the terminal
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;

        let corrected_x = u16::try_from(x.saturating_add(1)).unwrap();
        let corrected_y = u16::try_from(y.saturating_add(1)).unwrap();

        print!("{}", termion::cursor::Goto(corrected_x, corrected_y));
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key_result) = stdin().lock().keys().next() {
                return key_result;
            }
        }
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }
}
