use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many0, separated_list1},
    IResult,
};

use crate::{
    util::{Direction, Point},
    Puzzle,
};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let mut game = Game::parse(input)?;

        match game.play() {
            GameResult::Done(len) => Ok(len),
            GameResult::Loop => Err("Loop".into()),
        }
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let mut game = Game::parse(input)?;

        let mut count = 0;

        loop {
            if let Some((_, pos)) = game.next_move() {
                if game.map.get(&pos) == Some(&Tile::Empty)
                    && !game.visited_positions.contains(&pos)
                {
                    let mut modified_game = game.clone();
                    modified_game.map.insert(pos, Tile::Wall);

                    if modified_game.play() == GameResult::Loop {
                        count += 1;
                    }
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
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
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

#[derive(PartialEq, Eq)]
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
        let mut guard_pos = Point(0, 0);

        for (y, row) in tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == Tile::Guard {
                    guard_pos = Point(x as i64, y as i64);
                    map.insert(Point(x as i64, y as i64), Tile::Empty);
                } else {
                    map.insert(Point(x as i64, y as i64), tile);
                }
            }
        }

        Ok(Game::new(map, guard_pos))
    }

    fn next_move(&self) -> Option<(Direction, Point)> {
        let mut dir = self.dir;
        loop {
            let new_pos = self.guard_pos + dir;

            match self.map.get(&new_pos) {
                Some(Tile::Empty) => return Some((dir, new_pos)),
                Some(Tile::Wall) => dir = dir.rotate_clockwise(),
                _ => return None,
            }
        }
    }

    fn progress(&mut self) -> ProgressResult {
        if let Some((dir, pos)) = self.next_move() {
            if self.visited_state.contains(&(dir, pos)) {
                return ProgressResult::Loop;
            }

            self.guard_pos = pos;
            self.dir = dir;

            self.visited_positions.insert(pos);
            self.visited_state.insert((dir, pos));

            return ProgressResult::Continue;
        }

        return ProgressResult::End;
    }

    fn play(&mut self) -> GameResult {
        self.visited_positions.insert(self.guard_pos);

        loop {
            match self.progress() {
                ProgressResult::Continue => {}
                ProgressResult::End => break,
                ProgressResult::Loop => return GameResult::Loop,
            }
        }

        GameResult::Done(self.visited_positions.len() as u64)
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
