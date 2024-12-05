use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, (rules, updates)) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(updates
            .iter()
            .filter(|update| rules.iter().all(|rule| update.respects_rule(rule)))
            .map(|update| update.0[(update.0.len() - 1) / 2])
            .sum())
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
        let (_, (rules, updates)) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(updates
            .iter()
            .filter(|update| rules.iter().any(|rule| !update.respects_rule(rule)))
            .map(|update| update.sort_by_rules(&rules))
            .map(|update| update.0[(update.0.len() - 1) / 2])
            .sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, PartialEq)]
struct Rule(u64, u64);

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(separated_pair(u64, tag("|"), u64), |pair| {
        Rule(pair.0, pair.1)
    })(input)
}

#[derive(Debug, PartialEq)]
struct Update(Vec<u64>);

impl Update {
    fn respects_rule(&self, rule: &Rule) -> bool {
        let first_index = self
            .0
            .iter()
            .enumerate()
            .find(|(_, &x)| x == rule.0)
            .map(|(i, _)| i);

        let second_index = self
            .0
            .iter()
            .enumerate()
            .find(|(_, &x)| x == rule.1)
            .map(|(i, _)| i);

        if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
            first_index < second_index
        } else {
            true
        }
    }

    fn sort_by_rules(&self, rules: &[Rule]) -> Self {
        let mut list = self.0.clone();
        list.sort_by(|a, b| {
            if rules.iter().any(|rule| rule.0 == *a && rule.1 == *b) {
                return Ordering::Less;
            }

            if rules.iter().any(|rule| rule.1 == *a && rule.0 == *b) {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        Self(list)
    }
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    map(separated_list1(tag(","), u64), |list| Update(list))(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Update>)> {
    terminated(
        separated_pair(
            separated_list1(multispace1, parse_rule),
            multispace1,
            separated_list1(multispace1, parse_update),
        ),
        multispace0,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_rule() {
        let result = parse_rule("199|200");
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, Rule(199, 200));
    }

    #[test]
    fn test_parse_update() {
        let result = parse_update("75,29,13");
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, Update(vec![75, 29, 13]));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());
        let (remaining, (rules, updates)) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(rules.len(), 21);
        assert_eq!(rules[0], Rule(47, 53));
        assert_eq!(rules[20], Rule(53, 13));
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], Update(vec![75, 47, 61, 53, 29]));
        assert_eq!(updates[5], Update(vec![97, 13, 75, 29, 47]));
    }

    #[test]
    fn test_respects_rule() {
        let rule = Rule(75, 47);
        let update = Update(vec![75, 47, 61, 53, 29]);
        assert!(update.respects_rule(&rule));

        let rule = Rule(97, 75);
        let update = Update(vec![75, 97, 47, 61, 53]);
        assert!(!update.respects_rule(&rule));
    }

    #[test]
    fn test_sort_by_rules() {
        let rules = [Rule(61, 29), Rule(61, 13), Rule(29, 13)];
        let update = Update(vec![61, 13, 29]);
        assert_eq!(update.sort_by_rules(&rules), Update(vec![61, 29, 13]));
    }

    #[test]
    fn test_part1() {
        let result = Part1::solve_input(TEST_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 143);
    }

    #[test]
    fn test_part2() {
        let result = Part2::solve_input(TEST_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
    }
}
