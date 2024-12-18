mod game;

use std::collections::HashSet;

use game::Game;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

use crate::{util::Point, Puzzle};

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list0(
        newline,
        map(separated_pair(i64, tag(","), i64), |(x, y)| Point(x, y)),
    )(input)
}

fn part1(input: &str, size: Point, time: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let (_, points) = parse_input(input).map_err(|e| e.to_owned())?;

    let start = Point(0, 0);
    let end = size - Point(1, 1);

    let corrupted_points = points
        .iter()
        .take(time as usize)
        .copied()
        .collect::<HashSet<_>>();

    let game = Game::new(corrupted_points, size);

    Ok(game.find_path(start, end).ok_or("Path not found")?)
}

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        part1(INPUT, Point(71, 71), 1024).map(|e| e.to_string())
    }
}

fn part2(input: &str, size: Point) -> Result<Point, Box<dyn std::error::Error>> {
    let (_, points) = parse_input(input).map_err(|e| e.to_owned())?;

    let start = Point(0, 0);
    let end = size - Point(1, 1);

    let (mut l, mut r) = (0, points.len());

    while l < r {
        let mid = (l + r) / 2;

        let corrupted_points = points[..mid].iter().copied().collect::<HashSet<_>>();

        let game = Game::new(corrupted_points, size);

        if game.find_path(start, end).is_some() {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    return Ok(points[l - 1]);
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        part2(INPUT, Point(71, 71)).map(|p| p.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let input = "1,2\n1,2";

        assert_eq!(
            parse_input(input).unwrap(),
            ("", vec![Point(1, 2), Point(1, 2)])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, Point(7, 7), 12).unwrap(), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, Point(7, 7)).unwrap(), Point(6, 1));
    }
}
