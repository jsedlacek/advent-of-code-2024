mod game;

use game::Game;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (_, game) = Game::parse_input(INPUT)?;
        game.part1().map(|r| r.to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (_, game) = Game::parse_input(INPUT)?;
        game.part2().map(|r| r.to_string())
    }
}
