use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u64,
    combinator::map,
    multi::many0,
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, list) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(list
            .iter()
            .filter_map(|i| {
                if let Instruction::Mul(m) = i {
                    Some(m)
                } else {
                    None
                }
            })
            .map(|i| i.solve())
            .sum::<u64>())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, list) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut enabled = true;

        Ok(list
            .iter()
            .filter_map(|i| match i {
                Instruction::Mul(m) => {
                    if enabled {
                        Some(m)
                    } else {
                        None
                    }
                }
                Instruction::Void => None,
                Instruction::Do => {
                    enabled = true;
                    None
                }
                Instruction::Dont => {
                    enabled = false;
                    None
                }
            })
            .map(|i| i.solve())
            .sum::<u64>())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mul(u64, u64);

impl Mul {
    fn solve(self) -> u64 {
        self.0 * self.1
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Mul(Mul),
    Do,
    Dont,
    Void,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(alt((
        map(parse_mul, Instruction::Mul),
        map(tag("do()"), |_| Instruction::Do),
        map(tag("don't()"), |_| Instruction::Dont),
        map(take(1usize), |_| Instruction::Void),
    )))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = u64(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Mul(a, b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");

    #[test]
    fn test_parse_input() {
        let result = parse_input("mul(1,1)a;mul(2,2)");
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            result,
            vec![
                Instruction::Mul(Mul(1, 1)),
                Instruction::Void,
                Instruction::Void,
                Instruction::Mul(Mul(2, 2))
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT_2).unwrap(), 48);
    }
}
