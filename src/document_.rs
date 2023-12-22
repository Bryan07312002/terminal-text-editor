use crate::{editor::Position, row::Row};
use std::{fs::read_to_string, io::Error};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(path: &str) -> Result<Self, Error> {
        let content = read_to_string(path)?;

        let rows = content.lines().map(Row::from).collect::<Vec<Row>>();
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    // melhorar essa maneira de pegar mut refs
    pub fn insert_char(&mut self, pos: &Position, c: char) {
        match self.rows.get(pos.y) {
            Some(_) => self.rows[pos.y].insert_char(pos.x, c),
            None => {}
        };
    }

    // melhorar essa maneira de pegar mut refs
    pub fn remove_char(&mut self, pos: &Position) {
        match self.rows.get(pos.y) {
            Some(_) => self.rows[pos.y].remove_char(pos.x),
            None => {}
        };
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
