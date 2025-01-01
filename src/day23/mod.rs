use std::collections::{BTreeSet, HashMap, HashSet};

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
        Ok(part2(INPUT).join(",").to_string())
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list0(newline, separated_pair(alpha1, tag("-"), alpha1))(input)
}

pub fn part1(input: &str) -> u64 {
    let (_, pairs) = parse_input(input).unwrap();

    let mut neighbor_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut nodes = HashSet::new();

    for (a, b) in pairs.iter().copied() {
        neighbor_map.entry(a).or_default().insert(b);
        neighbor_map.entry(b).or_default().insert(a);
        nodes.insert(a);
        nodes.insert(b);
    }

    let mut trios = HashSet::new();

    for node in nodes {
        for &(a, b) in pairs.iter() {
            if a == node || b == node {
                continue;
            }
            let set = neighbor_map.get(node).unwrap();
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

    // Build the neighbor map
    let mut neighbor_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut nodes = HashSet::new();

    for (a, b) in pairs.iter().copied() {
        neighbor_map.entry(a).or_default().insert(b);
        neighbor_map.entry(b).or_default().insert(a);
        nodes.insert(a);
        nodes.insert(b);
    }

    // Collect all maximal cliques using the iterator
    let cliques = bron_kerbosch_pivot(&neighbor_map, &nodes);

    // Find the largest clique
    let max_clique = cliques.max_by_key(|clique| clique.len()).unwrap();

    // Return the largest clique nodes in sorted order
    let mut max_clique: Vec<String> = max_clique
        .into_iter()
        .map(|node| node.to_string())
        .collect();
    max_clique.sort();
    max_clique
}

fn bron_kerbosch_pivot<'a>(
    neighbor_map: &'a HashMap<&'a str, HashSet<&'a str>>,
    nodes: &'a HashSet<&'a str>,
) -> impl Iterator<Item = HashSet<&'a str>> {
    let mut stack = vec![(HashSet::new(), nodes.clone(), HashSet::new())];

    std::iter::from_fn(move || {
        while let Some((r, mut p, mut x)) = stack.pop() {
            if p.is_empty() && x.is_empty() {
                // Found a maximal clique
                return Some(r);
            }

            // Choose a pivot u from p ∪ x
            let u = p
                .union(&x)
                .max_by_key(|&&node| neighbor_map.get(node).unwrap().intersection(&p).count())
                .unwrap();
            let neighbors_u = neighbor_map.get(u).unwrap();

            // Candidates are p \ N(u)
            let candidates: Vec<&str> = p.difference(neighbors_u).cloned().collect();

            for &v in candidates.iter() {
                // r_new = r ∪ {v}
                let mut r_new = r.clone();
                r_new.insert(v);

                // p_new = p ∩ N(v)
                let neighbors_v = neighbor_map.get(v).unwrap();
                let p_new = p
                    .intersection(neighbors_v)
                    .cloned()
                    .collect::<HashSet<&str>>();

                // x_new = x ∩ N(v)
                let x_new = x
                    .intersection(neighbors_v)
                    .cloned()
                    .collect::<HashSet<&str>>();

                // Push (r_new, p_new, x_new) onto the stack
                stack.push((r_new, p_new, x_new));

                // Remove v from p and add to x
                p.remove(v);
                x.insert(v);
            }
        }

        None
    })
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
