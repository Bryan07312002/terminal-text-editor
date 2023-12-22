use crate::{document::Row, utils::Position};
use std::{fs::read_to_string, io::Error};

#[derive(Default, Clone)]
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

    pub fn lines_len(&self) -> usize {
        self.rows.len()
    }
}

impl From<Vec<&str>> for Document {
    fn from(value: Vec<&str>) -> Self {
        let rows: Vec<Row> = value.into_iter().map(Row::from).collect();
        Self { rows }
    }
}
