use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> u64 {
        part1(INPUT)
    }
}

fn part1(input: &str) -> u64 {
    let [mut list1, mut list2] = parse_input(input);

    list1.sort();

    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(&a, &b)| ((a as i64) - (b as i64)).abs() as u64)
        .sum()
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> u64 {
        part2(INPUT)
    }
}

fn part2(input: &str) -> u64 {
    let [list1, list2] = parse_input(input);

    list1
        .iter()
        .map(|a| list2.iter().filter(|b| a == *b).count() as u64 * a)
        .sum()
}

fn parse_input(input: &str) -> [Vec<u64>; 2] {
    let (list1, list2) = input
        .trim()
        .lines()
        .map(|line| {
            let numbers = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u64>())
                .collect::<Vec<_>>();

            if let [Ok(first), Ok(second)] = numbers[..] {
                (first, second)
            } else {
                panic!("Expected exactly two numbers per line");
            }
        })
        .unzip();

    [list1, list2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
