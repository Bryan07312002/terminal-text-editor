use std::cell::{RefCell, RefMut};
use std::io::Error;
use std::rc::Rc;

use termion::event::Key;

use crate::editor::{Mode, Window};
use crate::terminal::Terminal;
use crate::utils::{Direction, Position};

pub struct Control<'a> {
    cursor_position: Position,
    mode: Mode,
    attached_window: Option<usize>,
    windows: &'a Rc<RefCell<Vec<Window>>>,
}

impl<'a> Control<'a> {
    pub fn new(windows: &'a Rc<RefCell<Vec<Window>>>) -> Self {
        Self {
            cursor_position: Position { x: 0, y: 0 },
            mode: Mode::Normal,
            attached_window: None,
            windows,
        }
    }

    //-------------- Window utilities --------------//
    pub fn attach_window(&mut self, window_i: usize) {
        self.attached_window = Some(window_i);
    }

    pub fn borrow_attached_windows(&self) -> std::cell::Ref<'_, Vec<Window>> {
        self.windows.borrow()
    }

    pub fn borrow_mut_windows(&self) -> RefMut<'_, Vec<Window>> {
        self.windows.borrow_mut()
    }

    pub fn scroll_attached_window(&self, direction: &Direction, length: usize) {
        self.borrow_mut_windows()[self.attached_window.unwrap()].scroll(direction, length);
    }

    pub fn current_line_len(&self) -> usize {
        let line = self.borrow_attached_windows()[self.attached_window.unwrap()]
            .document_line_number_from_cursor(self.cursor_position.y);

        self.borrow_attached_windows()[self.attached_window.unwrap()]
            .document_row(line)
            .expect("line doesnt exists")
            .content
            .len()
    }
    //--------------------------------------------//

    pub fn position(&self) -> &Position {
        &self.cursor_position
    }

    pub fn listen_key() -> Result<Key, Error> {
        Terminal::read_key()
    }

    pub fn process_key(&mut self) -> Result<(), Error> {
        let key = Self::listen_key()?;

        match self.mode {
            Mode::Insert => todo!(),
            Mode::Normal => self.process_key_in_normal_mode(key),
            Mode::Command => todo!(),
        }

        Ok(())
    }

    pub fn process_key_in_normal_mode(&mut self, key: Key) {
        match key {
            Key::Backspace => todo!(),
            // basic moves
            Key::Char('h') => self.move_left(1),
            Key::Char('j') => self.move_down(1),
            Key::Char('k') => self.move_up(1),
            Key::Char('l') => self.move_right(1),

            Key::Char('A') => self.go_to_last_line_char(),

            // big jumps
            Key::Ctrl('d') => self.move_down_half_screen(),
            Key::Ctrl('u') => self.move_up_half_screen(),

            Key::Alt(_) => todo!(),
            Key::Ctrl(_) => todo!(),
            _ => todo!(),
        }
    }

    //-------------- Movement --------------//

    pub fn move_down(&mut self, length: usize) {
        let next_pos = Position {
            x: self.cursor_position.x,
            y: self.cursor_position.y + length,
        };

        let window_height = self.borrow_attached_windows()[self.attached_window.unwrap()]
            .size()
            .height as usize;

        if next_pos.y > window_height {
            self.scroll_attached_window(&Direction::Down, length);
            self.correct_x();
            return;
        }

        self.cursor_position = next_pos;
        self.correct_x();
    }

    pub fn move_up(&mut self, length: usize) {
        let next_y: isize = self.cursor_position.y as isize - length as isize;

        if next_y < 0 {
            self.scroll_attached_window(&Direction::Up, length);
        } else {
            self.cursor_position = Position {
                x: self.cursor_position.x,
                y: next_y as usize,
            };
        }

        self.correct_x();
    }

    pub fn move_right(&mut self, length: usize) {
        let next_pos = Position {
            x: self.cursor_position.x + length,
            y: self.cursor_position.y,
        };

        let window_width = self.borrow_attached_windows()[self.attached_window.unwrap()]
            .size()
            .width as usize;

        if self.current_line_len() < next_pos.x {
            return;
        }

        if next_pos.x > window_width {
            self.scroll_attached_window(&Direction::Right, length);
            return;
        }

        self.cursor_position = next_pos;
    }

    pub fn move_left(&mut self, length: usize) {
        let next_x: isize = self.cursor_position.x as isize + length as isize;

        if next_x < 0 {
            self.scroll_attached_window(&Direction::Left, 1);
            return;
        } else {
            self.cursor_position = Position {
                x: next_x as usize,
                y: self.cursor_position.y,
            }
        }
    }

    pub fn move_up_half_screen(&mut self) {
        let steps = (self.windows.borrow()[self.attached_window.unwrap()]
            .size()
            .height as f32
            * 0.7) as usize;

        self.move_up(steps);
    }

    pub fn move_down_half_screen(&mut self) {
        let steps = (self.windows.borrow()[self.attached_window.unwrap()]
            .size()
            .height
            / 2) as usize;

        self.move_down(steps);
    }

    pub fn correct_x(&mut self) {
        let line_len = self.current_line_len();
        if !(line_len < self.cursor_position.x) {
            return;
        }

        self.go_to_last_line_char();
    }

    pub fn go_to_last_line_char(&mut self) {
        let window_x_pos = self.borrow_attached_windows()[0].position().x;

        // get diffrence
        let correction_len: isize = self.current_line_len() as isize
            - self.cursor_position.x as isize
            + window_x_pos as isize;

        if correction_len > 0 {
            return self.move_right(correction_len as usize);
        } else {
            return self.move_left(correction_len as usize);
        }
    }
}
