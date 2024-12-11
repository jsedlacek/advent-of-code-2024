use std::collections::HashMap;
use std::error::Error;

use super::stone::Stone;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.evolve_stone(Stone::new(0), 1).unwrap(), 1);
        assert_eq!(game.evolve_stone(Stone::new(0), 2).unwrap(), 1);
        assert_eq!(game.evolve_stone(Stone::new(0), 3).unwrap(), 2);
    }

    #[test]
    fn test_play() {
        let mut game = Game::new();
        assert_eq!(
            game.evolve_stones(&[Stone::new(125), Stone::new(17)], 25)
                .unwrap(),
            55312
        );
        assert_eq!(
            game.evolve_stones(&[Stone::new(125), Stone::new(17)], 75)
                .unwrap(),
            65601038650482
        );
    }
}
