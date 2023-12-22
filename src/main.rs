#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod terminal;
mod utils;

use std::{cell::RefCell, env::args, rc::Rc};

use editor::{Control, Editor};
use terminal::Terminal;

fn main() {
    color_eyre::install().unwrap();

    let args: Vec<String> = args().collect();
    println!("{args:?}");

    let terminal = Terminal::default().unwrap();

    let windows = Rc::new(RefCell::new(Vec::new()));

    let mut control = Control::new(&windows);
    let mut e = Editor::new(&mut control, &windows, terminal).unwrap();

    if let Some(path) = args.get(1) {
        e.open_document(path).unwrap();
    };

    e.run();
}
