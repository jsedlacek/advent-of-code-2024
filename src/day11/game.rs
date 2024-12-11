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

    pub fn play(&mut self, numbers: Vec<u64>, rounds: u64) -> u64 {
        numbers.iter().map(|&n| self.stone_count(n, rounds)).sum()
    }

    fn stone_count(&mut self, number: u64, rounds: u64) -> u64 {
        if rounds == 0 {
            return 1;
        }

        if let Some(res) = self.stone_count_cache.get(&(number, rounds)) {
            return *res;
        }

        let res = Self::transform_stone(number)
            .into_iter()
            .map(|n| self.stone_count(n, rounds - 1))
            .sum();

        self.stone_count_cache.insert((number, rounds), res);

        res
    }

    fn transform_stone(number: u64) -> Vec<u64> {
        let mut result = Vec::new();

        match number {
            0 => {
                result.push(1);
            }
            num if num.to_string().len() % 2 == 0 => {
                let str = num.to_string();
                let (first, second) = str.split_at(str.len() / 2);
                result.push(first.parse::<u64>().unwrap());
                result.push(second.parse::<u64>().unwrap());
            }
            num => {
                result.push(num * 2024);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.stone_count(0, 1), 1);
        assert_eq!(game.stone_count(0, 2), 1);
        assert_eq!(game.stone_count(0, 3), 2);
    }

    #[test]
    fn test_part1() {
        let mut game = Game::new();
        assert_eq!(game.play(vec![125, 17], 25), 55312);
    }

    #[test]
    fn test_part2() {
        let mut game = Game::new();
        assert_eq!(game.play(vec![1], 75), 34840149002654);
    }
}
