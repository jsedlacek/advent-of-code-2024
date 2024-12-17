use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

use robot::Robot;

use crate::{
    util::{Direction, Point},
    Puzzle,
};

mod parse;
mod robot;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str, size: Point) -> Result<u64, Box<dyn Error>> {
        let (_, mut robots) = parse::parse_input(input).map_err(|e| e.to_owned())?;

        let round_count = 100;

        for robot in robots.iter_mut() {
            for _ in 0..round_count {
                robot.move_forward(size);
            }
        }

        let lt = robots
            .iter()
            .filter(|r| r.position.0 < size.0 / 2 && r.position.1 < size.1 / 2)
            .count();

        let rt = robots
            .iter()
            .filter(|r| r.position.0 > size.0 / 2 && r.position.1 < size.1 / 2)
            .count();

        let lb = robots
            .iter()
            .filter(|r| r.position.0 < size.0 / 2 && r.position.1 > size.1 / 2)
            .count();

        let rb = robots
            .iter()
            .filter(|r| r.position.0 > size.0 / 2 && r.position.1 > size.1 / 2)
            .count();

        Ok((lt * rt * lb * rb) as u64)
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Part1::solve_input(INPUT, Point(101, 103)).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        let (_, mut robots) = parse::parse_input(INPUT).map_err(|e| e.to_owned())?;

        let size = Point(101, 103);

        for round in 0.. {
            let area = biggest_area(&robots);

            if area >= 100 {
                return Ok(round.to_string());
            }

            for robot in robots.iter_mut() {
                robot.move_forward(size);
            }
        }

        panic!("Not found");
    }
}

fn biggest_area(robots: &[Robot]) -> u64 {
    let postitions = robots.iter().map(|r| r.position).collect::<HashSet<_>>();

    let mut processed_positions = HashSet::new();

    let mut max_count = 0;

    for &position in postitions.iter() {
        if processed_positions.contains(&position) {
            continue;
        }

        let mut count = 0;
        let mut queue = VecDeque::new();

        processed_positions.insert(position);
        queue.push_back(position);

        while let Some(position) = queue.pop_front() {
            for direction in Direction::all() {
                let neighbor_pos = position + direction;

                if !postitions.contains(&neighbor_pos) {
                    continue;
                }

                if !processed_positions.contains(&neighbor_pos) {
                    processed_positions.insert(neighbor_pos);
                    queue.push_back(neighbor_pos);
                    count += 1;
                }
            }
        }

        if count > max_count {
            max_count = count;
        }
    }

    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        let res = Part1::solve_input(TEST_INPUT, Point(7, 11));
        assert!(res.is_ok());

        let result = res.unwrap();
        assert_eq!(result, 12);
    }
}
