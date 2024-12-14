use crate::{util::Point, Puzzle};

mod parse;
mod robot;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str, size: Point) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, mut robots) = parse::parse_input(input).map_err(|e| e.to_owned())?;

        let round_count = 100;

        for robot in robots.iter_mut() {
            for _ in 0..round_count {
                robot.move_robot(size);
            }
        }

        let lt = robots
            .iter()
            .filter(|r| r.position.0 < size.0 / 2 && r.position.1 < size.1 / 2)
            .count();

        let rt = robots
            .iter()
            .filter(|r| r.position.0 > size.0 / 2 && r.position.1 < size.1 / 2)
            .count();

        let lb = robots
            .iter()
            .filter(|r| r.position.0 < size.0 / 2 && r.position.1 > size.1 / 2)
            .count();

        let rb = robots
            .iter()
            .filter(|r| r.position.0 > size.0 / 2 && r.position.1 > size.1 / 2)
            .count();

        Ok((lt * rt * lb * rb) as u64)
    }
}

impl Puzzle for Part1 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Part1::solve_input(INPUT, Point(101, 103))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    // #[test]
    // fn test_part_one_robot() {
    //     let res = Part1::solve_input("p=2,4 v=2,-3");
    //     assert!(res.is_ok());

    //     let result = res.unwrap();
    //     assert_eq!(result, 12);
    // }

    #[test]
    fn test_part1() {
        let res = Part1::solve_input(TEST_INPUT, Point(7, 11));
        assert!(res.is_ok());

        let result = res.unwrap();
        assert_eq!(result, 12);
    }
}
