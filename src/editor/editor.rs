use crate::{
    editor::Window,
    terminal::Terminal,
    utils::{Position, Size},
};
use std::{cell::RefCell, io::Error, rc::Rc};

use super::{Control, WindowManager};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor<'a> {
    should_quit: bool,
    terminal: Terminal,
    windows_manager: WindowManager<'a>,
    control: &'a mut Control<'a>,
}

impl<'a> Editor<'a> {
    pub fn new(
        control: &'a mut Control<'a>,
        windows: &'a Rc<RefCell<Vec<Window>>>,
        terminal: Terminal,
    ) -> Result<Self, Error> {
        let windows_manager = WindowManager::new(
            windows,
            Size {
                height: terminal.size().height,
                width: terminal.size().width,
            },
        );

        let res = Self {
            terminal,
            should_quit: false,
            windows_manager,
            control,
        };
        res.control.attach_window(0);

        Ok(res)
    }

    fn die(e: &Error) {
        panic!("{e}");
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                Self::die(&err);
            }

            if self.should_quit {
                break;
            }

            if let Err(err) = self.control.process_key() {
                Self::die(&err);
            };
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor();

        if self.should_quit {
            Terminal::clear_screen();
            Terminal::cursor_position(&Position { x: 0, y: 0 });
        } else {
            self.draw_windows();
            Terminal::cursor_position(self.control.position());

            // if has no windows render whelcome message
            if self.windows_manager.is_empty() {
                self.draw_welcome_message();
            }
        }

        Terminal::show_cursor();
        Terminal::flush()
    }

    pub fn draw_windows(&self) {
        let visible_buff = self.windows_manager.visible_area_buff();

        for line in 0..visible_buff.len() {
            Terminal::cursor_position(&Position { x: 0, y: line });
            Terminal::clear_current_line();
            print!("{}\r", visible_buff[line]);

            Terminal::cursor_position(&self.control.position());
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor --version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        println!("{welcome_message}\r");
    }
}
