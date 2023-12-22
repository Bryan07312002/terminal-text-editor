use std::cmp;

#[derive(Clone)]
pub struct Row {
    pub content: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            content: String::from(value),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.content.len());
        let start = cmp::min(start, end);

        self.content.get(start..end).unwrap_or_default().to_string()
    }

    pub fn insert_char(&mut self, pos: usize, c: char) {
        self.content.insert(pos, c)
    }

    pub fn remove_char(&mut self, pos: usize) {
        self.content.remove(pos);
    }

    pub fn next_word_index(&self, pos: usize) -> Option<usize> {
        self.content[pos..self.content.len()].find(" ")
    }
}
