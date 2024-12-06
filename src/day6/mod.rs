use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many0, separated_list1},
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn add(&self, dir: &Direction) -> Point {
        match dir {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, tiles) = parse_input(input).map_err(|e| e.to_owned())?;

        let mut map = HashMap::new();
        let mut guard_pos = Point::new(0, 0);

        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Guard {
                    guard_pos = Point::new(x as i64, y as i64);
                    map.insert(Point::new(x as i64, y as i64), Tile::Empty);
                } else {
                    map.insert(Point::new(x as i64, y as i64), *tile);
                }
            }
        }

        let mut set = HashSet::new();

        let mut dir = Direction::Up;

        while map.contains_key(&guard_pos) {
            set.insert(guard_pos.clone());
            if let Some((next_dir, next_pos)) = next_pos(&map, dir, guard_pos) {
                dir = next_dir;
                guard_pos = next_pos;
            } else {
                break;
            }
        }

        Ok(set.len() as u64)
    }
}

fn next_pos(map: &HashMap<Point, Tile>, dir: Direction, pos: Point) -> Option<(Direction, Point)> {
    let new_pos = pos.add(&dir);

    if let Some(Tile::Empty) = map.get(&new_pos) {
        return Some((dir, new_pos));
    }

    if let Some(Tile::Wall) = map.get(&new_pos) {
        return next_pos(map, dir.rotate(), pos);
    }

    return None;
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Guard,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, many0(parse_tile))(input)
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        map(tag("#"), |_| Tile::Wall),
        map(tag("."), |_| Tile::Empty),
        map(tag("^"), |_| Tile::Guard),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("#").unwrap(), ("", Tile::Wall));
        assert_eq!(parse_tile(".").unwrap(), ("", Tile::Empty));
        assert_eq!(parse_tile("^").unwrap(), ("", Tile::Guard));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());
        let map = result.unwrap().1;
        assert_eq!(map[0][0], Tile::Empty);
        assert_eq!(map[0][4], Tile::Wall);
        assert_eq!(map[6][4], Tile::Guard);
    }

    #[test]
    fn test_solve_input() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 41);
    }
}
