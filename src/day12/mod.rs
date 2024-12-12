use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    util::{Direction, Point},
    Puzzle,
};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    pub fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        let mut processed = HashSet::new();
        let mut set_list = Vec::new();

        for (&pos, item) in map.iter() {
            if processed.contains(&pos) {
                continue;
            }

            let mut set = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(pos);

            while let Some(queue_pos) = queue.pop_front() {
                if processed.contains(&queue_pos) {
                    continue;
                }

                set.insert(queue_pos);
                processed.insert(queue_pos);

                for dir in Direction::all() {
                    let neighbot_pos = queue_pos + dir;

                    if map.get(&neighbot_pos) != Some(item) {
                        continue;
                    }

                    if processed.contains(&neighbot_pos) {
                        continue;
                    }

                    queue.push_back(neighbot_pos);
                }
            }

            set_list.push(set);
        }

        set_list.iter().map(Self::fence_price).sum()
    }

    fn fence_price(fence: &HashSet<Point>) -> u64 {
        let area = fence.len() as u64;

        let perimeter = fence
            .iter()
            .map(|&pos| {
                Direction::all()
                    .into_iter()
                    .filter(|&dir| !fence.contains(&(pos + dir)))
                    .count() as u64
            })
            .sum::<u64>();

        area * perimeter
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(Self::solve_input(INPUT))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Tile(char);

impl Tile {
    fn new(c: char) -> Self {
        Self(c)
    }
}

fn parse_input(input: &str) -> HashMap<Point, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(x as i64, y as i64), Tile::new(c)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");
    const TEST_INPUT_3: &str = include_str!("test-input-3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 140);
        assert_eq!(Part1::solve_input(TEST_INPUT_2), 772);
        assert_eq!(Part1::solve_input(TEST_INPUT_3), 1930);
    }
}
