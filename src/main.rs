#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod terminal;
mod utils;

use std::{cell::RefCell, rc::Rc};

use document::Document;
use editor::{Control, Editor, Window};
use terminal::Terminal;
use utils::{Position, Size};

fn main() {
    color_eyre::install().unwrap();

    let terminal = Terminal::default().unwrap();
    let doc = Document::open("./testing.txt").unwrap();

    let windows = Rc::new(RefCell::new(vec![Window::new(
        doc,
        Size {
            width: terminal.size().width - 1,
            height: terminal.size().height,
        },
        Position { x: 1, y: 0 },
    )]));

    let mut control = Control::new(&windows);
    let mut e = Editor::new(&mut control, &windows, terminal).unwrap();
    e.run();
}
