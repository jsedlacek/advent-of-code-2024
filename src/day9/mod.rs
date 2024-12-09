mod disk;
mod parse;

use parse::parse_disk;

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, mut disk) = parse_disk(input).map_err(|e| e.to_owned())?;

        disk.defragment();

        Ok(disk.checksum())
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
        let (_, mut disk) = parse_disk(input).map_err(|e| e.to_owned())?;

        disk.defragment_v2();

        Ok(disk.checksum())
    }
}

impl Puzzle for Part2 {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Self::solve_input(INPUT)
    }
}

#[cfg(test)]
mod tests {
    use disk::{Disk, DiskBlock};

    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_checksum() {
        let disk = Disk::new(vec![
            DiskBlock::File(1),
            DiskBlock::Empty,
            DiskBlock::File(2),
        ]);

        assert_eq!(disk.checksum(), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Part1::solve_input(TEST_INPUT).unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Part2::solve_input(TEST_INPUT).unwrap(), 2858);
    }
}
