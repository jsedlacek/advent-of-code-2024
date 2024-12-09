use nom::{
    character::{complete::satisfy, is_digit},
    combinator::map,
    multi::many1,
    IResult,
};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize),
}

struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(many1(Self::parse_digit), |a| {
            let blocks = a
                .chunks(2)
                .enumerate()
                .flat_map(|(id, group)| {
                    let (file_size, free_size) = match *group {
                        [file_size] => (file_size, 0),
                        [file_size, free_size] => (file_size, free_size),
                        _ => unreachable!(),
                    };

                    let files = std::iter::repeat(Block::File(id)).take(file_size as usize);
                    let free_space = std::iter::repeat(Block::Empty).take(free_size as usize);

                    files.chain(free_space)
                })
                .collect::<Vec<_>>();

            Self { blocks }
        })(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, u64> {
        map(
            satisfy(|c| is_digit(c as u8)), // Check if the character is a digit
            |ch: char| ch.to_digit(10).unwrap() as u64,
        )(input)
    }

    fn defragment(&mut self) {
        let empty_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b == Block::Empty)
            .map(|(i, _)| i);

        let file_blocks = self
            .blocks
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| match b {
                Block::File(_) => true,
                _ => false,
            })
            .map(|(i, _)| i);

        for (i, j) in file_blocks.zip(empty_blocks) {
            if i > j {
                self.blocks.swap(i, j);
            }
        }
    }

    fn defragment_v2(&mut self) {
        let highest_file_index = self
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::File(id) => Some(*id),
                _ => None,
            })
            .max()
            .unwrap_or(0);

        for id in (0..=highest_file_index).rev() {
            if let Some(file_index) = self.blocks.iter().position(|b| *b == Block::File(id)) {
                let file_len = self
                    .blocks
                    .iter()
                    .skip(file_index)
                    .take_while(|b| **b == Block::File(id))
                    .count();

                if let Some(empty_index) = self.find_empty_block(file_len) {
                    if empty_index < file_index {
                        for i in 0..file_len {
                            self.blocks.swap(empty_index + i, file_index + i);
                        }
                    }
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, &b)| match b {
                Block::Empty => 0,
                Block::File(id) => i as u64 * id as u64,
            })
            .sum()
    }

    fn find_empty_block(&self, file_len: usize) -> Option<usize> {
        self.blocks
            .windows(file_len)
            .position(|window| window.iter().all(|b| *b == Block::Empty))
    }
}

pub struct Part1;

impl Part1 {
    fn solve_input(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let (_, mut disk) = Disk::parse(input).map_err(|e| e.to_owned())?;

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
        let (_, mut disk) = Disk::parse(input).map_err(|e| e.to_owned())?;

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
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let (_, disk) = Disk::parse("12").unwrap();

        assert_eq!(disk.blocks.len(), 3);
        assert_eq!(
            disk.blocks,
            vec![Block::File(0), Block::Empty, Block::Empty]
        );
    }

    #[test]
    fn test_checksum() {
        let disk = Disk {
            blocks: vec![Block::File(1), Block::Empty, Block::File(2)],
        };

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
