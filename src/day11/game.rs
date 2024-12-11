use std::collections::HashMap;
use std::error::Error;

pub struct Game {
    cache: HashMap<(u64, u64), u64>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn evolve_stones(
        &mut self,
        stones: &[u64],
        round_count: u64,
    ) -> Result<u64, Box<dyn Error>> {
        stones
            .iter()
            .map(|&stone| self.evolve_stone(stone, round_count))
            .sum()
    }

    fn evolve_stone(&mut self, stone: u64, round_count: u64) -> Result<u64, Box<dyn Error>> {
        if round_count == 0 {
            return Ok(1);
        }

        let key = (stone, round_count);

        if let Some(&res) = self.cache.get(&key) {
            return Ok(res);
        }

        let stones = Self::transform_stone(stone)?;
        let res = self.evolve_stones(&stones, round_count - 1)?;

        self.cache.insert(key, res);

        Ok(res)
    }

    fn transform_stone(stone: u64) -> Result<Vec<u64>, Box<dyn Error>> {
        if stone == 0 {
            return Ok(vec![1]);
        }

        let stone_str = stone.to_string();

        if stone_str.len() % 2 == 0 {
            let (first, second) = stone_str.split_at(stone_str.len() / 2);
            Ok(vec![first.parse()?, second.parse()?])
        } else {
            Ok(vec![stone.checked_mul(2024).ok_or("Overflow")?])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(Game::transform_stone(0).unwrap(), vec![1]);
        assert_eq!(Game::transform_stone(1).unwrap(), vec![2024]);
        assert_eq!(Game::transform_stone(2024).unwrap(), vec![20, 24]);
    }

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.evolve_stone(0, 1).unwrap(), 1);
        assert_eq!(game.evolve_stone(0, 2).unwrap(), 1);
        assert_eq!(game.evolve_stone(0, 3).unwrap(), 2);
    }

    #[test]
    fn test_play() {
        let mut game = Game::new();
        assert_eq!(game.evolve_stones(&[125, 17], 25).unwrap(), 55312);
        // assert_eq!(game.play(&[125, 17], 75).unwrap(), 65601038650482);
    }
}
