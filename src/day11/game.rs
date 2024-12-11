use std::collections::HashMap;
use std::error::Error;

pub struct Game {
    cache: HashMap<(Stone, u64), u64>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn evolve_stones(
        &mut self,
        stones: &[Stone],
        round_count: u64,
    ) -> Result<u64, Box<dyn Error>> {
        stones
            .iter()
            .map(|&stone| self.evolve_stone(stone, round_count))
            .sum()
    }

    fn evolve_stone(&mut self, stone: Stone, round_count: u64) -> Result<u64, Box<dyn Error>> {
        if round_count == 0 {
            return Ok(1);
        }

        let key = (stone, round_count);

        if let Some(&res) = self.cache.get(&key) {
            return Ok(res);
        }

        let stones = stone.transform()?;
        let res = self.evolve_stones(&stones, round_count - 1)?;

        self.cache.insert(key, res);

        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Stone(u64);

impl Stone {
    pub fn new(stone: u64) -> Self {
        Self(stone)
    }

    fn transform(&self) -> Result<Vec<Stone>, Box<dyn Error>> {
        if self.0 == 0 {
            return Ok(vec![Stone(1)]);
        }

        let stone_str = self.to_string();

        if stone_str.len() % 2 == 0 {
            let (first, second) = stone_str.split_at(stone_str.len() / 2);
            Ok(vec![Stone(first.parse()?), Stone(second.parse()?)])
        } else {
            Ok(vec![Stone(self.0.checked_mul(2024).ok_or("Overflow")?)])
        }
    }
}

impl ToString for Stone {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(Stone(0).transform().unwrap(), vec![Stone(1)]);
        assert_eq!(Stone(1).transform().unwrap(), vec![Stone(2024)]);
        assert_eq!(Stone(2024).transform().unwrap(), vec![Stone(20), Stone(24)]);
    }

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.evolve_stone(Stone(0), 1).unwrap(), 1);
        assert_eq!(game.evolve_stone(Stone(0), 2).unwrap(), 1);
        assert_eq!(game.evolve_stone(Stone(0), 3).unwrap(), 2);
    }

    #[test]
    fn test_play() {
        let mut game = Game::new();
        assert_eq!(
            game.evolve_stones(&[Stone(125), Stone(17)], 25).unwrap(),
            55312
        );
        assert_eq!(
            game.evolve_stones(&[Stone(125), Stone(17)], 75).unwrap(),
            65601038650482
        );
    }
}
