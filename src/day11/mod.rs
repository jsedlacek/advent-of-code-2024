mod game;
mod parse;

use std::error;

use game::Game;
use parse::parse_input;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn error::Error>> {
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        game.play(&numbers, 25)
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn error::Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn error::Error>> {
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        game.play(&numbers, 75)
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input("125 17").unwrap(), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input("1").unwrap(), 34840149002654);
    }
}
