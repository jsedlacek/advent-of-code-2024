use std::collections::{HashMap, HashSet};

use crate::{
    util::{Direction, Point},
    Puzzle,
};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    pub fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        let trailheads = map
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&point, _)| point)
            .collect::<Vec<_>>();

        trailheads
            .iter()
            .map(|point| {
                let mut targets = HashSet::new();
                Self::find_trails(&map, *point, &[], &mut targets);
                targets.len() as u64
            })
            .sum()
    }

    fn find_trails(
        map: &HashMap<Point, u64>,
        point: Point,
        trail: &[Point],
        targets: &mut HashSet<Point>,
    ) {
        let mut trail = trail.to_vec();
        trail.push(point);

        let height = map.get(&point);

        if let Some(height) = height {
            if *height == 9 {
                targets.insert(point);
            }

            for dir in Direction::all() {
                let next_point = point + dir;
                let next_height = map.get(&next_point);
                if next_height == Some(&(height + 1)) {
                    Self::find_trails(map, next_point, &trail, targets);
                }
            }
        }
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT))
    }
}

pub struct Part2;

impl Part2 {
    pub fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        let trailheads = map
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(&point, _)| point)
            .collect::<Vec<_>>();

        trailheads
            .iter()
            .map(|point| Self::find_trails(&map, *point))
            .sum()
    }

    fn find_trails(map: &HashMap<Point, u64>, point: Point) -> u64 {
        let height = map.get(&point);

        let mut count = 0;

        if let Some(height) = height {
            if *height == 9 {
                return 1;
            }

            for dir in Direction::all() {
                let next_point = point + dir;
                let next_height = map.get(&next_point);
                if next_height == Some(&(height + 1)) {
                    count += Self::find_trails(map, next_point);
                }
            }
        }

        count
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT))
    }
}

fn parse_input(input: &str) -> HashMap<Point, u64> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(y as i64, x as i64), c.to_digit(10).unwrap() as u64))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(map.len(), 64);
        assert_eq!(map.get(&Point(0, 0)), Some(&8));
        assert_eq!(map.get(&Point(7, 7)), Some(&2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT), 81);
    }
}
