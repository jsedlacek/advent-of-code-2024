pub struct Game {
    towels: Vec<String>,
}

impl Game {
    pub fn new(towels: &[&str]) -> Self {
        Self {
            towels: towels.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn design_count(&self, pattern: &str) -> u64 {
        let mut counts = vec![0u64; pattern.len() + 1];
        counts[0] = 1;

        for i in 0..pattern.len() {
            for towel in &self.towels {
                if pattern[i..].starts_with(towel) {
                    counts[i + towel.len()] += counts[i];
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
