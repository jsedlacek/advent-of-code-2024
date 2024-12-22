mod game;

use game::{part1, part2};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let numbers = parse_input(INPUT);
        Ok(part1(&numbers).to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let numbers = parse_input(INPUT);
        Ok(part2(&numbers).to_string())
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}
