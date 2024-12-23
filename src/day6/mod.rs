mod game;
mod parse;

use std::{collections::HashSet, error::Error};

use game::Game;
use parse::parse_input;

use crate::{util::Direction, Puzzle};

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
                let modified_game = game.clone_with_obstacle(*obstruction_pos);

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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1_solve_input() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 41);
    }

    #[test]
    fn test_part2_solve_input() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 6);
    }
}
