use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Tile(char);

impl Tile {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
