use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space0},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

pub struct Game {
    towels: Vec<String>,
    patterns: Vec<String>,
}

impl Game {
    fn new(towels: Vec<String>, patterns: Vec<String>) -> Self {
        Self { towels, patterns }
    }

    pub fn parse_input(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                separated_list0(tag(", "), delimited(space0::<&str, _>, alpha1, space0)),
                many1(newline),
                separated_list0(newline, alpha1),
            )),
            |(towels, _, patterns)| {
                Self::new(
                    towels.into_iter().map(|s| s.to_string()).collect(),
                    patterns.into_iter().map(|s| s.to_string()).collect(),
                )
            },
        )(input)
    }

    pub fn part1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut cache: HashMap<String, u64> = HashMap::new();

        Ok(self
            .patterns
            .iter()
            .filter(|p| self.design_count(&p, &mut cache) > 0)
            .count() as u64)
    }

    pub fn part2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut cache: HashMap<String, u64> = HashMap::new();

        Ok(self
            .patterns
            .iter()
            .map(|p| self.design_count(&p, &mut cache))
            .sum::<u64>())
    }

    fn design_count(&self, pattern: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if let Some(&result) = cache.get(pattern) {
            return result;
        }

        if pattern.is_empty() {
            return 1;
        }

        let count = self
            .towels
            .iter()
            .filter(|&towel| pattern.starts_with(towel))
            .map(|towel| self.design_count(&pattern[towel.len()..], cache))
            .sum::<u64>();

        cache.insert(pattern.to_string(), count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn design_count_test() {
        let game = Game::parse_input(TEST_INPUT).unwrap().1;

        let mut cache: HashMap<String, u64> = HashMap::new();

        assert_eq!(game.design_count("brwrr", &mut cache), 2);

        assert_eq!(game.design_count("bggr", &mut cache), 1);

        assert_eq!(game.design_count("gbbr", &mut cache), 4);

        assert_eq!(game.design_count("rrbgbr", &mut cache), 6);

        assert_eq!(game.design_count("ubwu", &mut cache), 0);

        assert_eq!(game.design_count("bwurrg", &mut cache), 1);

        assert_eq!(game.design_count("brgr", &mut cache), 2);

        assert_eq!(game.design_count("bbrgwb", &mut cache), 0);
    }

    #[test]
    fn part1_test() {
        let game = Game::parse_input(TEST_INPUT).unwrap().1;

        assert_eq!(game.part1().unwrap(), 6);
    }

    #[test]
    fn part2_test() {
        let game = Game::parse_input(TEST_INPUT).unwrap().1;
        assert_eq!(game.part2().unwrap(), 16);
    }
}
