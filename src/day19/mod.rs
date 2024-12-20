mod game;
mod parse;

use game::Game;
use parse::parse_input;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, (towels, patterns)) = parse_input(input).map_err(|e| e.to_owned())?;

        let game = Game::new(&towels);

        Ok(patterns
            .iter()
            .filter(|p| game.design_count(&p) > 0)
            .count() as u64)
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT)?.to_string())
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, (towels, patterns)) = parse_input(input).map_err(|e| e.to_owned())?;
        let game = Game::new(&towels);

        Ok(patterns.iter().map(|p| game.design_count(&p)).sum::<u64>())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT)?.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn part1_test() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 6);
    }

    #[test]
    fn part2_test() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 16);
    }
}
