use std::error::Error;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let [mut list1, mut list2] = parse_input(input)?;

        list1.sort();

        list2.sort();

        Ok(list1
            .iter()
            .zip(list2.iter())
            .map(|(&a, &b)| ((a as i64) - (b as i64)).abs() as u64)
            .sum())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let [list1, list2] = parse_input(input)?;

        Ok(list1
            .iter()
            .map(|a| list2.iter().filter(|b| a == *b).count() as u64 * a)
            .sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

fn parse_input(input: &str) -> Result<[Vec<u64>; 2], Box<dyn Error>> {
    let parsed_lines = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    let (list1, list2) = parsed_lines.into_iter().unzip();

    Ok([list1, list2])
}

fn parse_line(line: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let numbers = line
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    if let [a, b] = &numbers[..] {
        return Ok((*a, *b));
    } else {
        return Err(format!("Invalid line: {}", line).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 31);
    }
}
