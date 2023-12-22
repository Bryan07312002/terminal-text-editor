use crate::{
    document::Document,
    editor::Window,
    terminal::Terminal,
    utils::{Position, Size},
};
use std::{cell::RefCell, rc::Rc};

pub struct WindowManager<'a> {
    windows: &'a Rc<RefCell<Vec<Window>>>,
    size: Size,
}

impl<'a> WindowManager<'a> {
    pub fn new(windows: &'a Rc<RefCell<Vec<Window>>>, size: Size) -> Self {
        Self { windows, size }
    }

    pub fn clean_screen_buff(&self) -> Vec<String> {
        (0..self.size.height)
            .into_iter()
            .map(|_| {
                (0..self.size.width)
                    .map(|_| String::from(" "))
                    .collect::<String>()
            })
            .collect()
    }

    pub fn visible_area_buff(&self) -> Vec<String> {
        let mut screen = self.clean_screen_buff();
        for window in self.windows.borrow().iter() {
            let buff = window.visible_area_buff();
            let pos = window.position();
            let size = window.size();

            for row_i in 0..buff.len() {
                let start_at = pos.x;
                let end_at = pos.x + size.width as usize;

                let correction_y = pos.y + row_i;
                screen[correction_y].replace_range(start_at..end_at, &buff[row_i]);
            }
        }

        screen
    }

    pub fn new_window(&mut self, doc: Document) {
        let window = Window::new(
            doc,
            Size {
                width: 30,
                height: 10,
            },
            Position { x: 0, y: 0 },
        );

        self.windows.borrow_mut().push(window);
    }

    pub fn is_empty(&self) -> bool {
        self.windows.borrow().is_empty()
    }
}
