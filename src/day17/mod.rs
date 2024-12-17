use std::collections::HashSet;

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

pub struct Part2;

impl Part2 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, computer) = Computer::parse(input).map_err(|e| e.to_owned())?;

        let program = computer.get_program();

        let mut values = HashSet::from([0u64]);

        for i in (0..program.len()).rev() {
            values = values.iter().map(|v| v * 8).collect();

            let expected_output = program[i..].to_vec();

            let mut next_values = HashSet::new();

            for &value in &values {
                next_values.extend(Self::find_register_a(&computer, &expected_output, value));
            }

            values = next_values;
        }

        let &min_value = values.iter().min().unwrap();

        Ok(min_value)
    }

    fn find_register_a(
        computer: &Computer,
        expected_output: &[u64],
        starting_index: u64,
    ) -> Vec<u64> {
        let mut res = Vec::new();
        for register_a in starting_index..(starting_index + 8) {
            let mut computer = computer.clone_with_register_a(register_a as u64);
            let output = computer.run();

            if output == expected_output {
                res.push(register_a);
            }
        }

        return res;
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT).map(|res| res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(
            Part1::solve_input(TEST_INPUT).unwrap(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT_2).unwrap(), 117440);
    }
}
