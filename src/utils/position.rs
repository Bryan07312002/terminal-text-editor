#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Eq for Position {}
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
