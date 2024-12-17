use std::error::Error;

use crate::{
    util::{iter_2d, Point},
    Puzzle,
};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        const DIRECTIONS: [Point; 8] = [
            Point(0, 1),
            Point(1, 0),
            Point(0, -1),
            Point(-1, 0),
            Point(1, 1),
            Point(-1, -1),
            Point(1, -1),
            Point(-1, 1),
        ];

        iter_2d(&map)
            .flat_map(|(point, _)| DIRECTIONS.iter().map(move |&direction| (point, direction)))
            .filter(|&(point, direction)| has_match(&map, point, direction))
            .count() as u64
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Ok(Part1::solve_input(INPUT)).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> u64 {
        let map = parse_input(input);

        iter_2d(&map)
            .filter(|&(point, _)| has_match_part2(&map, point))
            .count() as u64
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Ok(Part2::solve_input(INPUT)).map(|res| res.to_string())
    }
}

fn has_match(map: &Vec<Vec<char>>, pos: Point, direction: Point) -> bool {
    const NEEDLE: &str = "XMAS";

    let Point(x, y) = pos;
    let Point(dx, dy) = direction;

    let new_x = x + dx * (NEEDLE.len() - 1) as i64;
    let new_y = y + dy * (NEEDLE.len() - 1) as i64;

    if new_x < 0 || new_x >= map[0].len() as i64 || new_y < 0 || new_y >= map.len() as i64 {
        return false;
    }

    for (i, c) in NEEDLE.chars().enumerate() {
        if map[(y as i64 + i as i64 * dy) as usize][(x as i64 + i as i64 * dx) as usize] != c {
            return false;
        }
    }

    true
}

fn has_match_part2(map: &Vec<Vec<char>>, pos: Point) -> bool {
    const NEEDLE: &str = "MAS";
    const NEEDLE_REV: &str = "SAM";

    let Point(x, y) = pos;
    let (x, y) = (x as usize, y as usize);

    if x + NEEDLE.len() > map[0].len() || y + NEEDLE.len() > map.len() {
        return false;
    }

    let first = [map[y][x], map[y + 1][x + 1], map[y + 2][x + 2]]
        .iter()
        .collect::<String>();

    let second = [map[y + 2][x], map[y + 1][x + 1], map[y][x + 2]]
        .iter()
        .collect::<String>();

    if (first != NEEDLE && first != NEEDLE_REV) || (second != NEEDLE && second != NEEDLE_REV) {
        return false;
    }

    true
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT;
        let result = parse_input(input);
        assert_eq!(result.len(), 10);
        assert_eq!(result[0].len(), 10);
        assert_eq!(result[0][0], 'M');
        assert_eq!(result[9][9], 'X');
    }

    #[test]
    fn test_has_match() {
        let map = parse_input(TEST_INPUT);

        assert!(has_match(&map, Point(5, 0), Point(1, 0)));
        assert!(has_match(&map, Point(3, 9), Point(1, -1)));
        assert!(has_match(&map, Point(3, 9), Point(-1, -1)));
    }

    #[test]
    fn test_has_match_part2() {
        let map = parse_input(TEST_INPUT);

        assert!(has_match_part2(&map, Point(1, 0)));
    }

    #[test]
    fn test_solve_input() {
        assert_eq!(Part1::solve_input(TEST_INPUT), 18);
    }

    #[test]
    fn test_solve_input_part_2() {
        assert_eq!(Part2::solve_input(TEST_INPUT), 9);
    }
}
