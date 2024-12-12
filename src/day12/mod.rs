mod parse;
mod region;
mod tile;

use std::error::Error;

use parse::parse_input;
use region::Region;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    pub fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, map) = parse_input(input).map_err(|e| e.to_owned())?;

        let regions = Region::find_regions(&map);

        Ok(regions.iter().map(|r| r.price_v1()).sum())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
}

pub struct Part2;

impl Part2 {
    pub fn solve_input(input: &str) -> Result<u64, Box<dyn Error>> {
        let (_, map) = parse_input(input).map_err(|e| e.to_owned())?;

        let regions = Region::find_regions(&map);

        Ok(regions.iter().map(|r| r.price_v2()).sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn Error>> {
        Self::solve_input(INPUT)
    }
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
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 140);
        assert_eq!(Part1::solve_input(TEST_INPUT_2).unwrap(), 772);
        assert_eq!(Part1::solve_input(TEST_INPUT_3).unwrap(), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 80);
        assert_eq!(Part2::solve_input(TEST_INPUT_2).unwrap(), 436);
        assert_eq!(Part2::solve_input(TEST_INPUT_3).unwrap(), 1206);
        assert_eq!(Part2::solve_input(TEST_INPUT_4).unwrap(), 236);
        assert_eq!(Part2::solve_input(TEST_INPUT_5).unwrap(), 368);
    }
}
