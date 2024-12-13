mod machine;
mod parse;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> u64 {
        let (_, machines) = parse::parse_input(input).unwrap();

        machines
            .iter()
            .map(|m| {
                let res = m.solve();
                dbg!(res);
                res
            })
            .sum()
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT))
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> u64 {
        let (_, mut machines) = parse::parse_input(input).unwrap();

        for machine in machines.iter_mut() {
            machine.increase_prices();
        }

        machines
            .iter()
            .map(|m| {
                let res = m.solve();
                dbg!(res);
                res
            })
            .sum()
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 480);
    }

    #[test]
    fn part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT), 875318608908);
    }
}
