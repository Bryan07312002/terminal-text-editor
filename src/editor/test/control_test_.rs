mod test {
    use crate::document::Document;

    fn default_doc(value: Option<Vec<&str>>) -> Document {
        match value {
            Some(rows) => Document::from(rows),
            None => Document::from(vec!["Hello World"]),
        }
    }

    #[test]
    fn test_create_new_control() {
        use crate::editor::Control;
        use std::{cell::RefCell, rc::Rc};

        let windows = Rc::new(RefCell::new(Vec::new()));
        let _ = Control::new(&windows);
    }

    #[ignore]
    #[test]
    fn test_should_not_panic_when_has_no_windows() {
        use crate::editor::Control;
        use std::{cell::RefCell, rc::Rc};

        let windows = Rc::new(RefCell::new(Vec::new()));
        let mut control = Control::new(&windows);
        control.move_up(10);
    }

    #[test]
    fn test_should_should_move_down() {
        use crate::{
            editor::{Control, Window},
            utils::{Position, Size},
        };
        use std::{cell::RefCell, rc::Rc};

        let window = Window::new(
            default_doc(Some(vec!["", ""])),
            Size {
                width: 10,
                height: 10,
            },
            Position { x: 0, y: 0 },
        );

        let windows = Rc::new(RefCell::new(vec![window]));
        let mut control = Control::new(&windows);
        control.attach_window(0);

        control.move_down(1);

        assert_eq!(control.position().clone(), Position { x: 0, y: 1 });
    }

    #[test]
    fn test_should_should_move_down_() {
        use crate::{
            editor::{Control, Window},
            utils::{Position, Size},
        };
        use std::{cell::RefCell, rc::Rc};

        let window = Window::new(
            default_doc(Some(vec!["", ""])),
            Size {
                width: 10,
                height: 10,
            },
            Position { x: 0, y: 0 },
        );

        let windows = Rc::new(RefCell::new(vec![window]));
        let mut control = Control::new(&windows);
        control.attach_window(0);
        control.move_down(2);

        assert_eq!(control.position().clone(), Position { x: 0, y: 1 });
    }
}
