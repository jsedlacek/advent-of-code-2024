use combination::*;
use std::collections::{HashMap, HashSet};

use crate::{util::Point, Puzzle};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        let set: HashSet<Point> =
            HashSet::from_iter(map.iter().enumerate().flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(y, _)| Point(x as i64, y as i64))
            }));

        let mut antenas = HashMap::new();
        for (x, row) in map.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if let Tile::Antenna(c) = tile {
                    antenas
                        .entry(*c)
                        .or_insert_with(Vec::new)
                        .push(Point(x as i64, y as i64));
                }
            }
        }

        let mut antinodes = HashSet::new();

        for points in antenas.values() {
            for i in combine::from_vec_at(points, 2) {
                if let [a, b] = i[..] {
                    let diff = b - a;

                    for antinode in [a - diff, b + diff] {
                        if set.contains(&antinode) {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }

        antinodes.len() as u64
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Part1::solve_input(INPUT))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Antenna(char),
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    _ => Tile::Antenna(c),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 14);
    }
}