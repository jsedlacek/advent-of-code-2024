use crate::Puzzle;

mod parse;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, equations) = parse::parse(input).map_err(|e| e.to_owned())?;

        Ok(equations
            .iter()
            .filter(|eq| eq.is_valid())
            .map(|eq| eq.result)
            .sum())
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[derive(Debug, PartialEq)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        match self.numbers[..] {
            [n] => self.result == n,
            [.., n] => {
                let rest_numbers = self.numbers[..self.numbers.len() - 1].to_vec();

                if n <= self.result {
                    let add_eq = Equation {
                        result: self.result - n,
                        numbers: rest_numbers.clone(),
                    };
                    if add_eq.is_valid() {
                        return true;
                    }
                }

                if self.result % n == 0 {
                    let mul_eq = Equation {
                        result: self.result / n,
                        numbers: rest_numbers,
                    };

                    if mul_eq.is_valid() {
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
        .is_valid());

        assert!(!Equation {
            result: 1,
            numbers: vec![2],
        }
        .is_valid());

        assert!(Equation {
            result: 2,
            numbers: vec![1, 1],
        }
        .is_valid());

        assert!(Equation {
            result: 8,
            numbers: vec![2, 2, 2],
        }
        .is_valid());
    }

    #[test]
    fn test_solve_input() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 3749);
    }
}
