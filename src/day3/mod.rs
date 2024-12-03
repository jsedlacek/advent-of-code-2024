use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, u64},
    combinator::{flat_map, map, map_opt},
    multi::{self, many0},
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, list) = parse_input(input).unwrap();

        Ok(list.iter().map(|i| i.0 * i.1).sum::<u64>())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mul(u64, u64);

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)) -> Mul
// ignore everything other than mul
fn parse_input(input: &str) -> IResult<&str, Vec<Mul>> {
    let (input, list) = many0(alt((map(parse_mul, Some), map(take(1u64), |_| None))))(input)?;

    let list = list.iter().filter_map(|x| *x).collect::<Vec<_>>();

    Ok((input, list))
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

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 161);
    }
}
