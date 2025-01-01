use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{count, many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

enum Item {
    Lock(Vec<u64>),
    Key(Vec<u64>),
}

type Lock = Vec<u64>;
type Key = Vec<u64>;

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Lock>, Vec<Key>)> {
    map(
        separated_list1(
            many1(newline),
            alt((map(parse_lock, Item::Lock), map(parse_key, Item::Key))),
        ),
        |items| {
            let locks = items
                .iter()
                .filter_map(|item| match item {
                    Item::Lock(lock) => Some(lock.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let keys = items
                .iter()
                .filter_map(|item| match item {
                    Item::Key(key) => Some(key.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            (locks, keys)
        },
    )(input)
}

fn parse_lock(input: &str) -> IResult<&str, Lock> {
    map(
        tuple((
            terminated(count(tag("#"), 5), newline),
            separated_list1(newline, many1(parse_tile)),
        )),
        |(_, tiles)| {
            let mut res = vec![0; tiles[0].len()];
            for row in tiles.iter() {
                for (x, &tile) in row.iter().enumerate() {
                    if !tile {
                        res[x] += 1;
                    }
                }
            }
            res
        },
    )(input)
}

fn parse_key(input: &str) -> IResult<&str, Key> {
    map(
        tuple((
            terminated(count(tag("."), 5), newline),
            separated_list1(newline, many1(parse_tile)),
        )),
        |(_, tiles)| {
            let mut res = vec![0; tiles[0].len()];
            for row in tiles.iter() {
                for (x, &tile) in row.iter().enumerate() {
                    if tile {
                        res[x] += 1;
                    }
                }
            }
            res
        },
    )(input)
}

fn parse_tile(input: &str) -> IResult<&str, bool> {
    alt((map(tag("#"), |_| true), map(tag("."), |_| false)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let (_, (locks, keys)) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(locks.len(), 2);
        assert_eq!(keys.len(), 3);

        assert_eq!(locks[0], vec![6, 1, 3, 2, 3]);

        assert_eq!(keys[0], vec![6, 1, 3, 2, 4]);
    }
}
