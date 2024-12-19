use std::collections::HashMap;

pub struct Game {
    towels: Vec<String>,
    cache: HashMap<String, u64>,
}

impl Game {
    pub fn new(towels: &[&str]) -> Self {
        Self {
            towels: towels.iter().map(|&s| s.to_string()).collect(),
            cache: HashMap::new(),
        }
    }

    // design_count and design_count_logic are separated to split the ownership of cache
    // from the ownership of towels.
    pub fn design_count(&mut self, pattern: &str) -> u64 {
        Self::design_count_logic(&self.towels, pattern, &mut self.cache)
    }

    fn design_count_logic(
        towels: &[String],
        pattern: &str,
        cache: &mut HashMap<String, u64>,
    ) -> u64 {
        if let Some(&result) = cache.get(pattern) {
            return result;
        }

        if pattern.is_empty() {
            return 1;
        }

        let count = towels
            .iter()
            .filter(|&towel| pattern.starts_with(towel))
            .map(|towel| Self::design_count_logic(towels, &pattern[towel.len()..], cache))
            .sum();

        cache.insert(pattern.to_string(), count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_count_test() {
        let mut game = Game::new(&["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);

        assert_eq!(game.design_count("brwrr"), 2);

        assert_eq!(game.design_count("bggr"), 1);

        assert_eq!(game.design_count("gbbr"), 4);

        assert_eq!(game.design_count("rrbgbr"), 6);

        assert_eq!(game.design_count("ubwu"), 0);

        assert_eq!(game.design_count("bwurrg"), 1);

        assert_eq!(game.design_count("brgr"), 2);

        assert_eq!(game.design_count("bbrgwb"), 0);
    }
}
