mod game;
mod parse;

use std::error::Error;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        let (_, game) = parse::parse_input(INPUT)?;
        let speedups = game.find_cheat_speedups(2);

        Ok(speedups
            .iter()
            .filter(|(speedup, _)| **speedup >= 100)
            .map(|(_, count)| count)
            .sum::<u64>()
            .to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        let (_, game) = parse::parse_input(INPUT)?;
        let speedups = game.find_cheat_speedups(20);

        Ok(speedups
            .iter()
            .filter(|(speedup, _)| **speedup >= 100)
            .map(|(_, count)| count)
            .sum::<u64>()
            .to_string())
    }
}
