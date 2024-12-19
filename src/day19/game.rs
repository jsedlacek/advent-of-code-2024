use std::collections::HashMap;

pub struct Game<'a> {
    towels: &'a [&'a str],
    cache: HashMap<String, u64>,
}

impl<'a> Game<'a> {
    pub fn new(towels: &'a [&'a str]) -> Self {
        Self {
            towels,
            cache: HashMap::new(),
        }
    }

    pub fn design_count(&mut self, pattern: &str) -> u64 {
        if let Some(&result) = self.cache.get(pattern) {
            return result;
        }

        if pattern.is_empty() {
            return 1;
        }

        let count = self
            .towels
            .iter()
            .filter(|&&towel| pattern.starts_with(towel))
            .map(|&towel| self.design_count(&pattern[towel.len()..]))
            .sum();

        self.cache.insert(pattern.to_string(), count);

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
