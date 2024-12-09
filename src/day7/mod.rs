use crate::Puzzle;

mod parse;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, equations) = parse::parse(input).map_err(|e| e.to_owned())?;

        Ok(equations
            .iter()
            .filter(|eq| eq.is_valid(Version::V1))
            .map(|eq| eq.result)
            .sum())
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
        let (_, equations) = parse::parse(input).map_err(|e| e.to_owned())?;

        Ok(equations
            .iter()
            .filter(|eq| eq.is_valid(Version::V2))
            .map(|eq| eq.result)
            .sum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, PartialEq)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Version {
    V1,
    V2,
}

impl Equation {
    fn is_valid(&self, version: Version) -> bool {
        match &self.numbers[..] {
            [n] => self.result == *n,
            [rest_numbers @ .., n] => {
                if *n <= self.result {
                    let add_eq = Equation {
                        result: self.result - n,
                        numbers: rest_numbers.to_vec(),
                    };
                    if add_eq.is_valid(version) {
                        return true;
                    }
                }

                if self.result % n == 0 {
                    let mul_eq = Equation {
                        result: self.result / n,
                        numbers: rest_numbers.to_vec(),
                    };

                    if mul_eq.is_valid(version) {
                        return true;
                    }
                }

                let result_str = self.result.to_string();
                if version == Version::V2
                    && *n != self.result
                    && result_str.ends_with(&n.to_string())
                {
                    let next_result = result_str[..result_str.len() - n.to_string().len()]
                        .parse::<u64>()
                        .unwrap();

                    let concat_eq = Equation {
                        result: next_result,
                        numbers: rest_numbers.to_vec(),
                    };

                    if concat_eq.is_valid(version) {
                        return true;
                    }
                }

                return false;
            }

            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_is_valid() {
        assert!(Equation {
            result: 1,
            numbers: vec![1],
        }
        .is_valid(Version::V1));

        assert!(!Equation {
            result: 1,
            numbers: vec![2],
        }
        .is_valid(Version::V1));

        assert!(Equation {
            result: 2,
            numbers: vec![1, 1],
        }
        .is_valid(Version::V1));

        assert!(Equation {
            result: 8,
            numbers: vec![2, 2, 2],
        }
        .is_valid(Version::V1));

        assert!(!Equation {
            result: 192,
            numbers: vec![17, 8, 14],
        }
        .is_valid(Version::V1));

        assert!(Equation {
            result: 192,
            numbers: vec![17, 8, 14],
        }
        .is_valid(Version::V2));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 3749);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 11387);
    }
}
