#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Tile(char);

impl Tile {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}
