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

        let regions = find_regions(&map);

        regions.iter().map(Self::fence_price).sum()
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

pub struct Part2;

impl Part2 {
    pub fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        let regions = find_regions(&map);

        regions.iter().map(Self::fence_price).sum()
    }

    fn fence_price(fence: &HashSet<Point>) -> u64 {
        let area = fence.len() as u64;

        let mut fence_vec = fence.iter().cloned().collect::<Vec<_>>();
        fence_vec.sort();

        let mut processed = HashSet::new();
        let mut sides = 0;

        for pos in fence_vec {
            for dir in Direction::all() {
                if fence.contains(&(pos + dir)) {
                    continue;
                }

                if [pos + dir.rotate_left(), pos + dir.rotate_right()]
                    .iter()
                    .all(|&p| !processed.contains(&(p, dir)))
                {
                    sides += 1;
                }

                processed.insert((pos, dir));
            }
        }

        area * sides
    }
}

impl Puzzle for Part2 {
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

fn find_regions(map: &HashMap<Point, Tile>) -> Vec<HashSet<Point>> {
    let mut processed = HashSet::new();
    let mut regions = Vec::new();

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

        regions.push(set);
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");
    const TEST_INPUT_3: &str = include_str!("test-input-3.txt");
    const TEST_INPUT_4: &str = include_str!("test-input-4.txt");
    const TEST_INPUT_5: &str = include_str!("test-input-5.txt");

    #[test]
    fn fence_price() {
        assert_eq!(
            Part2::fence_price(&HashSet::from_iter([Point(0, 0)])),
            (1 * 4)
        );
        assert_eq!(
            Part2::fence_price(&HashSet::from_iter([Point(0, 0), Point(0, 1)])),
            (2 * 4)
        );
        assert_eq!(
            Part2::fence_price(&HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])),
            (3 * 6)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 140);
        assert_eq!(Part1::solve_input(TEST_INPUT_2), 772);
        assert_eq!(Part1::solve_input(TEST_INPUT_3), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT), 80);
        assert_eq!(Part2::solve_input(TEST_INPUT_2), 436);
        assert_eq!(Part2::solve_input(TEST_INPUT_3), 1206);
        assert_eq!(Part2::solve_input(TEST_INPUT_4), 236);
        assert_eq!(Part2::solve_input(TEST_INPUT_5), 368);
    }
}
