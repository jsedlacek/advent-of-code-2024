mod parse;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
};

use parse::parse_input;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, (rule_set, updates)) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(updates
            .iter()
            .filter(|update| rule_set.is_update_valid(update))
            .map(|update| update.find_mid_item())
            .sum())
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
        let (_, (rule_set, updates)) = parse_input(input).map_err(|e| e.to_owned())?;

        Ok(updates
            .iter()
            .filter(|update| !rule_set.is_update_valid(update))
            .map(|update| update.sort_by_rules(&rule_set))
            .map(|update| update.find_mid_item())
            .sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[derive(Debug, PartialEq)]
struct Rule(u64, u64);

#[derive(Debug, PartialEq)]
struct Update(Vec<u64>);

impl Update {
    fn sort_by_rules(&self, rule_set: &RuleSet) -> Self {
        let mut list = self.0.clone();
        list.sort_by(|a, b| rule_set.compare(a, b));
        Self(list)
    }

    fn find_mid_item(&self) -> u64 {
        self.0[(self.0.len() - 1) / 2]
    }
}

struct RuleSet {
    precedence: HashSet<(u64, u64)>,
}

impl RuleSet {
    fn new(rules: &[Rule]) -> Self {
        let precedence = rules.iter().map(|rule| (rule.0, rule.1)).collect();
        Self { precedence }
    }

    fn compare(&self, a: &u64, b: &u64) -> Ordering {
        if self.precedence.contains(&(*a, *b)) {
            return Ordering::Less;
        }
        if self.precedence.contains(&(*b, *a)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }

    fn is_update_valid(&self, update: &Update) -> bool {
        let positions: HashMap<u64, usize> =
            update.0.iter().enumerate().map(|(i, &x)| (x, i)).collect();

        for &(a, b) in &self.precedence {
            if let (Some(&pos_a), Some(&pos_b)) = (positions.get(&a), positions.get(&b)) {
                if pos_a >= pos_b {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_respects_update() {
        let rules = vec![Rule(75, 47), Rule(97, 75)];
        let rule_set = RuleSet::new(&rules);

        let update1 = Update(vec![75, 47, 61, 53, 29]);
        assert!(rule_set.is_update_valid(&update1));

        let update2 = Update(vec![75, 97, 47, 61, 53]);
        assert!(!rule_set.is_update_valid(&update2));
    }

    #[test]
    fn test_sort_by_rules() {
        let rules = vec![Rule(61, 29), Rule(61, 13), Rule(29, 13)];
        let rule_set = RuleSet::new(&rules);

        let update = Update(vec![61, 13, 29]);
        assert_eq!(update.sort_by_rules(&rule_set), Update(vec![61, 29, 13]));
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
