use computer::Computer;

use crate::Puzzle;

mod computer;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (_, mut computer) = Computer::parse(input).map_err(|e| e.to_owned())?;
        let output = computer.run();
        Ok(output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(","))
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1_solve_input() {
        assert_eq!(
            Part1::solve_input(TEST_INPUT).unwrap(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
