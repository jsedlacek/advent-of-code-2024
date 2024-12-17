mod machine;
mod parse;

use std::error::Error;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, machines) = parse::parse_input(input).map_err(|e| e.to_owned())?;

        Ok(machines.iter().map(|m| m.solve()).sum())
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
        let (_, mut machines) = parse::parse_input(input).map_err(|e| e.to_owned())?;

        for machine in machines.iter_mut() {
            machine.increase_prices();
        }

        Ok(machines.iter().map(|m| m.solve()).sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 480);
    }

    #[test]
    fn part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 875318608908);
    }
}
