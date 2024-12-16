use parse::parse_input;

use crate::Puzzle;

mod game;
mod parse;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, game) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(game.part1())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, game) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(game.part2())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}
