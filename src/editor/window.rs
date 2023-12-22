use crate::{
    document::{Document, Row},
    utils::{Direction, Position, Size},
};
use std::ops::Range;

struct VisibleArea {
    x_range: (usize, usize),
    y_range: (usize, usize),
}

impl VisibleArea {
    pub fn y_range(&self) -> Range<usize> {
        self.y_range.0..self.y_range.1
    }
}

pub struct Window {
    doc: Document,
    size: Size,
    visible_area: VisibleArea,
    position: Position,
}

impl Window {
    pub fn new(doc: Document, size: Size, position: Position) -> Self {
        Self {
            doc,
            visible_area: VisibleArea {
                x_range: (0, size.width as usize),
                y_range: (0, size.height as usize),
            },
            size,
            position,
        }
    }

    pub fn scroll(&mut self, direction: &Direction, length: usize) {
        match direction {
            Direction::Up => {
                let initial_space = self.visible_area.y_range.1 - self.visible_area.y_range.0;
                let y0 = self.visible_area.y_range.0.saturating_sub(length);
                let y1 = self.visible_area.y_range.1.saturating_sub(length);

                if initial_space == (y1 - y0) {
                    self.visible_area.y_range.0 = y0;
                    self.visible_area.y_range.1 = y1;
                }
            }
            Direction::Down => {
                // let initial_space = //self.visible_area.y_range.1 - self.visible_area.y_range.0;
                let mut y0 = self.visible_area.y_range.0.saturating_add(length);
                let mut y1 = self.visible_area.y_range.1.saturating_add(length);
                let line_len = self.doc.lines_len();

                // if doc has no more lines got to last line
                if line_len < y1 {
                    y1 = y1 - line_len;
                    y0 = y0 - y1;
                }

                if self.size().height as usize == (y1.saturating_sub(y0)) {
                    self.visible_area.y_range.0 = y0;
                    self.visible_area.y_range.1 = y1;
                }
            }

            // TODO: LEFT AND RIGHT ARE NOT TESTED
            Direction::Left => {
                let initial_space = self.visible_area.x_range.1 - self.visible_area.x_range.0;
                let x0 = self.visible_area.x_range.0.saturating_sub(length);
                let x1 = self.visible_area.x_range.1.saturating_sub(length);

                if initial_space == (x1 - x0) {
                    self.visible_area.x_range.0 = x0;
                    self.visible_area.x_range.1 = x1;
                }
            }
            Direction::Right => {
                let initial_space = self.size.width as usize; //self.visible_area.x_range.1 - self.visible_area.x_range.0;
                let x0 = self.visible_area.x_range.0.saturating_add(length);
                let x1 = self.visible_area.x_range.1.saturating_add(length);
                if initial_space == (x1 - x0) {
                    self.visible_area.x_range.0 = x0;
                    self.visible_area.x_range.1 = x1;
                }
            }
        }
    }

    pub fn visible_area_buff(&self) -> Vec<String> {
        // take all rows in range
        let visible_text_y: Vec<&Row> = self
            .visible_area
            .y_range()
            .map(|i| self.doc.row(i))
            .take_while(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();

        let visible_text_x: Vec<String> = visible_text_y
            .iter()
            .map(|row| row.render(self.visible_area.x_range.0, self.visible_area.x_range.1))
            .collect();

        visible_text_x
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn document_line_number_from_cursor(&self, pos_y: usize) -> usize {
        self.visible_area.y_range.0 + pos_y
    }

    pub fn document_row(&self, line: usize) -> Option<&Row> {
        self.doc.row(line)
    }
}

mod test {
    use crate::utils::Position;

    #[test]
    fn should_be_visible() {
        use crate::{document::Document, editor::window::Window, utils::Size};

        let w = Window::new(
            Document::from(vec!["line1", "line2", "line3", "line4"]),
            Size {
                width: 2,
                height: 2,
            },
            Position::default(),
        );

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 2);
        assert_eq!(buff[0].len(), 2);
    }

    #[test]
    fn should_be_visible_not_even_size() {
        use crate::{document::Document, editor::window::Window, utils::Size};

        let w = Window::new(
            Document::from(vec!["line1", "line2", "line3", "line4"]),
            Size {
                width: 2,
                height: 4,
            },
            Position::default(),
        );

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 4);
        assert_eq!(buff[0].len(), 2);
    }

    #[test]
    fn should_scroll_down() {
        use crate::{
            document::Document,
            editor::window::Window,
            utils::{Direction, Size},
        };
        let text_buff = vec!["line1", "line2", "line3", "line4"];

        let mut w = Window::new(
            Document::from(text_buff.clone()),
            Size {
                width: 6,
                height: 3,
            },
            Position::default(),
        );

        w.scroll(&Direction::Down, 1);

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 3);
        assert_eq!(buff[0].len(), text_buff[0].len());
        assert_eq!(buff, text_buff[1..text_buff.len()]);
    }

    #[test]
    fn should_not_scroll_down() {
        // cant scroll down more than page size
        use crate::{
            document::Document,
            editor::window::Window,
            utils::{Direction, Size},
        };
        let text_buff = vec!["line1", "line2", "line3", "line4"];

        let mut w = Window::new(
            Document::from(text_buff.clone()),
            Size {
                width: 6,
                height: 3,
            },
            Position::default(),
        );

        w.scroll(&Direction::Down, 1);
        w.scroll(&Direction::Down, 10);

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 3);
        assert_eq!(buff[0].len(), text_buff[0].len());
        assert_eq!(buff, text_buff[1..text_buff.len()]);
    }

    #[test]
    fn should_scroll_up() {
        use crate::{
            document::Document,
            editor::window::Window,
            utils::{Direction, Size},
        };
        let text_buff = vec!["line1", "line2", "line3", "line4"];

        let mut w = Window::new(
            Document::from(text_buff.clone()),
            Size {
                width: 6,
                height: 3,
            },
            Position::default(),
        );

        w.scroll(&Direction::Down, 1);
        w.scroll(&Direction::Up, 1);

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 3);
        assert_eq!(buff[0].len(), text_buff[0].len());
        assert_eq!(buff, text_buff[0..text_buff.len() - 1]);
    }

    #[test]
    fn should_not_scroll_up() {
        use crate::{
            document::Document,
            editor::window::Window,
            utils::{Direction, Size},
        };
        let text_buff = vec!["line1", "line2", "line3", "line4"];

        let mut w = Window::new(
            Document::from(text_buff.clone()),
            Size {
                width: 6,
                height: 3,
            },
            Position::default(),
        );

        w.scroll(&Direction::Up, 1);

        let buff = w.visible_area_buff();
        assert_eq!(buff.len(), 3);
        assert_eq!(buff[0].len(), text_buff[0].len());
        assert_eq!(buff, text_buff[0..text_buff.len() - 1]);
    }
}
