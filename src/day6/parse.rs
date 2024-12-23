use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::util::{iter_2d, Point, PointRange};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Guard,
}

pub fn parse_input(input: &str) -> IResult<&str, (HashSet<Point>, PointRange, Point)> {
    map(separated_list1(newline, many1(parse_tile)), |tiles| {
        let walls = iter_2d(&tiles)
            .filter(|(_, &tile)| tile == Tile::Wall)
            .map(|(pos, _)| pos)
            .collect();

        let range = PointRange::new(
            Point(0, 0),
            Point(tiles[0].len() as i64, tiles.len() as i64),
        );

        let guard_pos = iter_2d(&tiles)
            .find(|(_, &tile)| tile == Tile::Guard)
            .map(|(pos, _)| pos)
            .unwrap();

        (walls, range, guard_pos)
    })(input)
}

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    alt((
        map(tag("#"), |_| Tile::Wall),
        map(tag("."), |_| Tile::Empty),
        map(tag("^"), |_| Tile::Guard),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("#").unwrap(), ("", Tile::Wall));
        assert_eq!(parse_tile(".").unwrap(), ("", Tile::Empty));
        assert_eq!(parse_tile("^").unwrap(), ("", Tile::Guard));
    }

    #[test]
    fn test_parse_input() {
        let (_, (walls, _, _)) = parse_input(TEST_INPUT).unwrap();
        assert!(walls.contains(&Point(4, 0)));
    }
}
