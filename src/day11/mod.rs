mod game;
mod parse;

use game::Game;
use parse::parse_input;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        Ok(game.play(numbers, 25))
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
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        Ok(game.play(numbers, 75))
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}
