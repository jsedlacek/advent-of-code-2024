use std::error::Error;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        parse_input(input)
            .map(|lines| lines.iter().filter(|&numbers| is_safe(numbers)).count() as u64)
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
        parse_input(input)
            .map(|lines| lines.iter().filter(|&numbers| is_safe_v2(numbers)).count() as u64)
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    Ok(line
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}

fn is_safe(numbers: &[u64]) -> bool {
    let mut last_direction = None;

    for window in numbers.windows(2) {
        let diff = window[1] as i64 - window[0] as i64;

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        let current_direction = diff.signum();

        if let Some(last_direction) = last_direction {
            if last_direction != current_direction {
                return false;
            }
        }

        last_direction = Some(current_direction);
    }

    true
}

fn is_safe_v2(numbers: &[u64]) -> bool {
    if is_safe(numbers) {
        return true;
    }

    for index in 0..numbers.len() {
        let mut numbers_copy = numbers.to_vec();
        numbers_copy.remove(index);
        if is_safe(&numbers_copy) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("1 2 3\n4 5 6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]],
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

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 4);
    }
}
