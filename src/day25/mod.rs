use crate::Puzzle;

mod parse;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (_, (locks, keys)) = parse::parse_input(INPUT)?;

        Ok(format!("{}", part1(locks, keys)))
    }
}

fn part1(locks: Vec<Vec<u64>>, keys: Vec<Vec<u64>>) -> u64 {
    locks
        .iter()
        .flat_map(|lock| {
            keys.iter().filter(|key| {
                lock.iter()
                    .zip(key.iter())
                    .all(|(lock_height, key_height)| lock_height >= key_height)
            })
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        let (_, (locks, keys)) = parse::parse_input(TEST_INPUT).unwrap();

        assert_eq!(part1(locks, keys), 3);
    }
}
