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

    pub fn evolve_stones(&mut self, numbers: &[u64], rounds: u64) -> Result<u64, Box<dyn Error>> {
        numbers.iter().map(|&n| self.evolve_stone(n, rounds)).sum()
    }

    fn evolve_stone(&mut self, number: u64, rounds: u64) -> Result<u64, Box<dyn Error>> {
        if rounds == 0 {
            return Ok(1);
        }

        let key = (number, rounds);

        if let Some(&res) = self.cache.get(&key) {
            return Ok(res);
        }

        let res = self.evolve_stones(&Self::transform_stone(number)?, rounds - 1)?;

        self.cache.insert(key, res);

        Ok(res)
    }

    fn transform_stone(number: u64) -> Result<Vec<u64>, Box<dyn Error>> {
        if number == 0 {
            return Ok(vec![1]);
        }

        let number_str = number.to_string();
        if number_str.len() % 2 == 0 {
            let (first, second) = number_str.split_at(number_str.len() / 2);
            Ok(vec![first.parse()?, second.parse()?])
        } else {
            Ok(vec![number.checked_mul(2024).ok_or("Overflow")?])
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
