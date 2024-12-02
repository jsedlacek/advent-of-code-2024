use std::error::Error;

use crate::Puzzle;

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let lines = parse_input(input)?;

        Ok(lines
            .iter()
            .map(|numbers| if is_safe(numbers) { 1 } else { 0 })
            .sum())
    }
}

const INPUT: &str = include_str!("input.txt");

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let lines = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}

fn parse_line(line: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let numbers = line
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<_, _>>()?;

    Ok(numbers)
}

fn is_safe(numbers: &[u64]) -> bool {
    let mut last_number = None;
    let mut last_direction = None;

    for number in numbers {
        if let Some(last_number) = last_number {
            let diff = *number as i64 - last_number as i64;

            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            if let Some(last_direction) = last_direction {
                if last_direction != diff.signum() {
                    return false;
                }
            }

            last_direction = Some(diff.signum());
        }

        last_number = Some(*number);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("1 2 3\n4 5 6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6],],
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1 2 3").unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[1, 2, 3]));
        assert!(!is_safe(&[1, 4, 8]));
        assert!(!is_safe(&[1, 3, 2]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 2);
    }
}
