mod game;

use game::{part1, part2};
use nom::{
    character::complete::{multispace0, newline, u64},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (_, numbers) = parse_input(INPUT)?;

        Ok(part1(numbers).to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (_, numbers) = parse_input(INPUT)?;

        Ok(part2(numbers)
            .ok_or("Part 2 solution not found")?
            .to_string())
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    all_consuming(delimited(
        multispace0,
        separated_list1(newline, u64),
        multispace0,
    ))(input)
}
