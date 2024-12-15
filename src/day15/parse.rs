use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

use crate::util::{iter_2d, Direction};

use super::game::{Game, GameBox};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ParseTile {
    Wall,
    Box,
    Robot,
}

pub fn parse_input(input: &str) -> IResult<&str, Game> {
    map(
        tuple((parse_tile_list, many0(newline), parse_instruction_list)),
        |(tiles, _, instructions)| {
            let boxes = iter_2d(&tiles)
                .filter(|(_, &tile)| tile == Some(ParseTile::Box))
                .map(|(point, _)| GameBox::new(HashSet::from([point])))
                .collect::<Vec<_>>();

            let wall_positions = iter_2d(&tiles)
                .filter(|(_, &tile)| tile == Some(ParseTile::Wall))
                .map(|(point, _)| point)
                .collect::<HashSet<_>>();

            let robot_position = iter_2d(&tiles)
                .find(|(_, &tile)| tile == Some(ParseTile::Robot))
                .map(|(point, _)| point)
                .unwrap();

            Game::new(boxes, wall_positions, robot_position, instructions)
        },
    )(input)
}

fn parse_tile_list(input: &str) -> IResult<&str, Vec<Vec<Option<ParseTile>>>> {
    separated_list0(newline, many0(parse_tile))(input)
}

fn parse_instruction_list(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(delimited(many0(newline), parse_instruction, many0(newline)))(input)
}

fn parse_tile(input: &str) -> IResult<&str, Option<ParseTile>> {
    alt((
        map(tag("#"), |_| Some(ParseTile::Wall)),
        map(tag("O"), |_| Some(ParseTile::Box)),
        map(tag("@"), |_| Some(ParseTile::Robot)),
        map(tag("."), |_| None),
    ))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("^"), |_| Direction::Up),
        map(tag(">"), |_| Direction::Right),
        map(tag("v"), |_| Direction::Down),
        map(tag("<"), |_| Direction::Left),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::util::Point;

    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("^"), Ok(("", Direction::Up)));
        assert_eq!(parse_instruction(">"), Ok(("", Direction::Right)));
        assert_eq!(parse_instruction("v"), Ok(("", Direction::Down)));
        assert_eq!(parse_instruction("<"), Ok(("", Direction::Left)));
    }

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("#"), Ok(("", Some(ParseTile::Wall))));
        assert_eq!(parse_tile("O"), Ok(("", Some(ParseTile::Box))));
        assert_eq!(parse_tile("@"), Ok(("", Some(ParseTile::Robot))));
        assert_eq!(parse_tile("."), Ok(("", None)));
    }

    #[test]
    fn test_parse_tile_list() {
        let (_, tiles) = parse_tile_list("#@.O").unwrap();
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].len(), 4);
    }

    #[test]
    fn test_parse_instruction_list() {
        let (_, instructions) = parse_instruction_list("^^").unwrap();
        assert_eq!(instructions.len(), 2);

        let (_, instructions) = parse_instruction_list("^\n^").unwrap();
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_parse_input() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(game.robot_position, Point(4, 4),)
    }
}
