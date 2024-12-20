mod game;
mod parse;

use std::error::Error;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        let (_, game) = parse::parse_input(INPUT)?;
        let speedups = game.find_cheat_speedups();

        Ok(speedups
            .iter()
            .filter(|(speedup, _)| **speedup >= 100)
            .map(|(_, count)| count)
            .sum::<u64>()
            .to_string())
    }
}
