use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    iter::once,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("{}", part1(INPUT)))
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("{}", part2(INPUT).join(",")))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(newline, separated_pair(alpha1, tag("-"), alpha1))(input)
}

pub fn part1(input: &str) -> u64 {
    let (_, pairs) = parse_input(input).unwrap();

    let nodes: HashSet<&str> =
        HashSet::from_iter(pairs.iter().flat_map(|&(a, b)| once(a).chain(once(b))));

    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (a, b) in pairs.iter() {
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }

    let mut trios = HashSet::new();

    for node in nodes {
        for &(a, b) in pairs.iter() {
            if a == node || b == node {
                continue;
            }
            let set = map.get(node).unwrap();
            if set.contains(&a) && set.contains(&b) {
                let trio = BTreeSet::from([a, b, node]);
                trios.insert(trio);
            }
        }
    }

    trios
        .iter()
        .filter(|trio| trio.iter().any(|node| node.starts_with("t")))
        .count() as u64
}

pub fn part2(input: &str) -> Vec<String> {
    let (_, pairs) = parse_input(input).unwrap();

    let nodes = BTreeSet::from_iter(pairs.iter().flat_map(|&(a, b)| once(a).chain(once(b))));

    let mut neighbor_map: HashMap<&str, BTreeSet<&str>> = HashMap::new();

    for (a, b) in pairs.iter() {
        neighbor_map.entry(a).or_default().insert(b);
        neighbor_map.entry(b).or_default().insert(a);
    }

    let mut queue = VecDeque::from_iter(nodes.iter().map(|&node| BTreeSet::from([node])));
    let mut results = BTreeSet::from_iter(queue.iter().cloned());

    while let Some(set) = queue.pop_front() {
        let neighbor_nodes = set
            .iter()
            .map(|node| neighbor_map.get(node).unwrap())
            .fold(None, |a, b| match a {
                None => Some(b.clone()),
                Some(a) => Some(BTreeSet::from_iter(a.intersection(&b).copied())),
            })
            .unwrap();

        for node in (&neighbor_nodes - &set).iter() {
            let mut next_set = set.clone();
            next_set.insert(node);

            if !results.contains(&next_set) {
                queue.push_back(next_set.clone());
                results.insert(next_set);
            }
        }
    }

    let res = results.iter().max_by_key(|set| set.len()).unwrap();

    res.iter().map(|node| node.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let (_, pairs) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(pairs.first().unwrap(), &("kh", "tc"));
        assert_eq!(pairs.last().unwrap(), &("td", "yn"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), vec!["co", "de", "ka", "ta"]);
    }
}
