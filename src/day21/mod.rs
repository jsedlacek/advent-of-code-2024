mod game;

use game::{Digit, Game};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        get_complexity(INPUT, 3).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        get_complexity(INPUT, 26).map(|res| res.to_string())
    }
}

fn parse_input(input: &str) -> Vec<Vec<Digit>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Digit::from_char(c))
                .flatten()
                .collect()
        })
        .collect()
}

fn get_complexity(input: &str, max_level: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let mut game = Game::new(max_level);

    let codes = parse_input(input);

    let complexity = codes
        .into_iter()
        .map(|code| {
            let sequence_len = game.get_sequence_len(code.iter().copied());

            let num = Game::get_numeric_part(code.into_iter());

            num * sequence_len as u64
        })
        .sum::<u64>();

    Ok(complexity)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_get_complexity() {
        let complexity = get_complexity(TEST_INPUT, 3).unwrap();

        assert_eq!(complexity, 126384);
    }
}
