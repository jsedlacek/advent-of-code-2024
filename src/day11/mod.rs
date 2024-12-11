use std::collections::HashMap;

use nom::{
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        Ok(game.play(numbers, 25))
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, numbers) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut game = Game::new();
        Ok(game.play(numbers, 75))
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

pub struct Game {
    stone_count_cache: HashMap<(u64, u64), u64>,
}

impl Game {
    fn new() -> Self {
        Self {
            stone_count_cache: HashMap::new(),
        }
    }

    fn play(&mut self, numbers: Vec<u64>, rounds: u64) -> u64 {
        numbers
            .clone()
            .iter()
            .map(|&n| self.stone_count(n, rounds))
            .sum()
    }

    fn stone_count(&mut self, number: u64, rounds: u64) -> u64 {
        if rounds == 0 {
            return 1;
        }

        if let Some(res) = self.stone_count_cache.get(&(number, rounds)) {
            return *res;
        }

        let res = Self::transform_stone(number)
            .into_iter()
            .map(|n| self.stone_count(n, rounds - 1))
            .sum();

        self.stone_count_cache.insert((number, rounds), res);

        res
    }

    fn transform_stone(number: u64) -> Vec<u64> {
        let mut result = Vec::new();

        match number {
            0 => {
                result.push(1);
            }
            num if num.to_string().len() % 2 == 0 => {
                let str = num.to_string();
                let (first, second) = str.split_at(str.len() / 2);
                result.push(first.parse::<u64>().unwrap());
                result.push(second.parse::<u64>().unwrap());
            }
            num => {
                result.push(num * 2024);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 456 789";
        let expected = vec![123, 456, 789];
        assert_eq!(parse_input(input), Ok(("", expected)));
    }

    #[test]
    fn test_stone_count() {
        let mut game = Game::new();
        assert_eq!(game.stone_count(0, 1), 1);
        assert_eq!(game.stone_count(0, 2), 1);
        assert_eq!(game.stone_count(0, 3), 2);
    }

    #[test]
    fn test_part1() {
        let mut game = Game::new();
        assert_eq!(game.play(vec![125, 17], 25), 55312);
    }

    #[test]
    fn test_part2() {
        let mut game = Game::new();
        assert_eq!(game.play(vec![1], 75), 34840149002654);
    }
}
