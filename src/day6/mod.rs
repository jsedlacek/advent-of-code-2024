use std::{collections::HashSet, error::Error};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::{
    util::{iter_2d, Direction, Point, PointRange},
    Puzzle,
};

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, (walls, range, guard_pos)) = parse_input(input).map_err(|e| e.to_owned())?;

        let game = Game::new(walls, range);

        Ok(game
            .iter((guard_pos, Direction::Up))
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len() as u64)
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
        let (_, (walls, range, guard_pos)) = parse_input(input).map_err(|e| e.to_owned())?;

        let game = Game::new(walls, range);

        let guard = (guard_pos, Direction::Up);

        let positions = game.iter(guard).collect::<Vec<_>>();

        let mut visited = HashSet::new();

        let obstructions = positions
            .windows(2)
            .map(|w| (w[0], w[1].0))
            .filter(|&(_, pos)| visited.insert(pos));

        Ok(obstructions
            .filter(|(guard, obstruction_pos)| {
                let mut modified_game = game.clone();
                modified_game.walls.insert(*obstruction_pos);

                modified_game.is_loop(*guard)
            })
            .count() as u64)
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[derive(Debug, Clone)]
struct Game {
    walls: HashSet<Point>,
    range: PointRange,
}

impl Game {
    fn new(walls: HashSet<Point>, range: PointRange) -> Game {
        Game { walls, range }
    }

    fn iter(&self, guard: (Point, Direction)) -> impl Iterator<Item = (Point, Direction)> + '_ {
        let (mut pos, mut dir) = guard;

        return std::iter::once(guard).chain(std::iter::from_fn(move || loop {
            let new_pos = pos + dir;

            if !self.range.contains(new_pos) {
                return None;
            }

            if !self.walls.contains(&new_pos) {
                pos = new_pos;
                return Some((pos, dir));
            }

            dir = dir.rotate_clockwise();
        }));
    }

    fn is_loop(&self, guard: (Point, Direction)) -> bool {
        let mut visited_state = HashSet::new();

        for state in self.iter(guard) {
            if !visited_state.insert(state) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Guard,
}

fn parse_input(input: &str) -> IResult<&str, (HashSet<Point>, PointRange, Point)> {
    map(separated_list1(newline, many1(parse_tile)), |tiles| {
        let walls = iter_2d(&tiles)
            .filter(|(_, &tile)| tile == Tile::Wall)
            .map(|(pos, _)| pos)
            .collect();

        let range = PointRange::new(
            Point(0, 0),
            Point(tiles[0].len() as i64, tiles.len() as i64),
        );

        let guard_pos = iter_2d(&tiles)
            .find(|(_, &tile)| tile == Tile::Guard)
            .map(|(pos, _)| pos)
            .unwrap();

        (walls, range, guard_pos)
    })(input)
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
        let (_, (walls, _, _)) = parse_input(TEST_INPUT).unwrap();
        assert!(walls.contains(&Point(4, 0)));
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
