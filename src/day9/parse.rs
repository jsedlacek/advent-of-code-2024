use nom::{
    character::{complete::satisfy, is_digit},
    combinator::map,
    multi::many1,
    IResult,
};

use super::disk::{Block, Disk};

pub fn parse_disk(input: &str) -> IResult<&str, Disk> {
    map(many1(parse_digit), |a| {
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

        Disk::new(blocks)
    })(input)
}

fn parse_digit(input: &str) -> IResult<&str, u64> {
    map(
        satisfy(|c| is_digit(c as u8)), // Check if the character is a digit
        |ch: char| ch.to_digit(10).unwrap() as u64,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (_, disk) = parse_disk("12").unwrap();

        assert_eq!(disk.blocks.len(), 3);
        assert_eq!(
            disk.blocks,
            vec![Block::File(0), Block::Empty, Block::Empty]
        );
    }
}
