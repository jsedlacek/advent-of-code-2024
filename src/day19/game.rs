use std::collections::{HashMap, HashSet};

pub struct Game {
    towels_by_length: HashMap<usize, HashSet<String>>,
}

impl Game {
    pub fn new(towels: &[&str]) -> Self {
        let mut towels_by_len: HashMap<usize, HashSet<String>> = HashMap::new();

        for towel in towels {
            let length = towel.len();
            towels_by_len
                .entry(length)
                .or_default()
                .insert(towel.to_string());
        }

        Self {
            towels_by_length: towels_by_len,
        }
    }

    pub fn design_count(&self, pattern: &str) -> u64 {
        let mut counts = vec![0u64; pattern.len() + 1];
        counts[0] = 1;

        for i in 0..pattern.len() {
            for (&length, towels) in self.towels_by_length.iter() {
                if let Some(sub_pattern) = pattern.get(i..i + length) {
                    if towels.contains(sub_pattern) {
                        counts[i + length] += counts[i];
                    }
                }
            }
        }

        counts[pattern.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_count_test() {
        let game = Game::new(&["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);

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
