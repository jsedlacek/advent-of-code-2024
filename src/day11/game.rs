use std::collections::HashMap;

pub struct Game {
    stone_count_cache: HashMap<(u64, u64), u64>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            stone_count_cache: HashMap::new(),
        }
    }

    pub fn play(&mut self, numbers: &[u64], rounds: u64) -> u64 {
        numbers.iter().map(|&n| self.stone_count(n, rounds)).sum()
    }

    fn stone_count(&mut self, number: u64, rounds: u64) -> u64 {
        if rounds == 0 {
            return 1;
        }

        let key = (number, rounds);

        if let Some(&res) = self.stone_count_cache.get(&key) {
            return res;
        }

        let res = Self::transform_stone(number)
            .into_iter()
            .map(|n| self.stone_count(n, rounds - 1))
            .sum();

        self.stone_count_cache.insert(key, res);

        res
    }

    fn transform_stone(number: u64) -> Vec<u64> {
        let number_str = number.to_string();

        if number == 0 {
            vec![1]
        } else if number_str.len() % 2 == 0 {
            let (first, second) = number_str.split_at(number_str.len() / 2);
            vec![
                first.parse::<u64>().unwrap(),
                second.parse::<u64>().unwrap(),
            ]
        } else {
            vec![number * 2024]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(Game::transform_stone(0), vec![1]);
        assert_eq!(Game::transform_stone(1), vec![2024]);
        assert_eq!(Game::transform_stone(2024), vec![20, 24]);
    }

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.stone_count(0, 1), 1);
        assert_eq!(game.stone_count(0, 2), 1);
        assert_eq!(game.stone_count(0, 3), 2);
    }

    #[test]
    fn test_play() {
        let mut game = Game::new();
        assert_eq!(game.play(&[125, 17], 25), 55312);
        assert_eq!(game.play(&[125, 17], 75), 65601038650482);
    }
}
