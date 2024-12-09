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

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let mut game = Game::parse(input)?;

        match game.play()? {
            GameResult::Done(len) => Ok(len),
            GameResult::Loop => Err("Loop".into()),
        }
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let mut game = Game::parse(input)?;

        let mut count = 0;
        loop {
            let wall_pos = game.guard_pos.add(&game.dir);

            if let Some(Tile::Empty) = game.map.get(&wall_pos) {
                let mut modified_game = game.clone();
                modified_game.map.insert(wall_pos, Tile::Wall);

                if let Ok(GameResult::Loop) = modified_game.play() {
                    count += 1;
                }
            }

            if let ProgressResult::End = game.progress() {
                break;
            }
        }

        Ok(count)
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, Clone)]
struct Game {
    map: HashMap<Point, Tile>,
    guard_pos: Point,
    dir: Direction,
    visited_positions: HashSet<Point>,
    visited_state: HashSet<(Direction, Point)>,
}

enum ProgressResult {
    Continue,
    End,
    Loop,
}

enum GameResult {
    Done(u64),
    Loop,
}

impl Game {
    fn new(map: HashMap<Point, Tile>, guard_pos: Point) -> Game {
        Game {
            map,
            guard_pos,
            dir: Direction::Up,
            visited_positions: HashSet::new(),
            visited_state: HashSet::new(),
        }
    }

    fn parse(input: &str) -> Result<Game, Box<dyn std::error::Error>> {
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

        Ok(Game::new(map, guard_pos))
    }

    fn progress(&mut self) -> ProgressResult {
        loop {
            let new_pos = self.guard_pos.add(&self.dir);

            if let Some(Tile::Empty) = self.map.get(&new_pos) {
                if self.visited_state.contains(&(self.dir, new_pos)) {
                    return ProgressResult::Loop;
                }

                self.guard_pos = new_pos;
                self.visited_positions.insert(new_pos);
                self.visited_state.insert((self.dir, new_pos));
                return ProgressResult::Continue;
            }

            if let Some(Tile::Wall) = self.map.get(&new_pos) {
                self.dir = self.dir.rotate();
                continue;
            }

            return ProgressResult::End;
        }
    }

    fn play(&mut self) -> Result<GameResult, Box<dyn std::error::Error>> {
        loop {
            match self.progress() {
                ProgressResult::Continue => {}
                ProgressResult::End => break,
                ProgressResult::Loop => return Ok(GameResult::Loop),
            }
        }

        self.visited_positions.insert(self.guard_pos);
        Ok(GameResult::Done(self.visited_positions.len() as u64))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    fn test_part1_solve_input() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 41);
    }

    #[test]
    fn test_part2_solve_input() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 6);
    }
}
