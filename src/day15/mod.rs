use crate::Puzzle;

mod game;
mod parse;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(&self, input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, mut game) = parse::parse_input(input).map_err(|e| e.to_owned())?;

        Ok(game.play())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        self.solve_input(INPUT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");

    #[test]
    fn test_part1() {
        let result = Part1.solve_input(TEST_INPUT).unwrap();
        assert_eq!(result, 10092);

        let result = Part1.solve_input(TEST_INPUT_2).unwrap();
        assert_eq!(result, 2028);
    }
}
