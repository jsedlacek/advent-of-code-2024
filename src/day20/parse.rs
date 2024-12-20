use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    IResult,
};

use crate::util::iter_2d;

use super::game::Game;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ParseTile {
    Empty,
    Wall,
    Exit,
    Start,
}

pub fn parse_input(input: &str) -> IResult<&str, Game> {
    map_res(
        separated_list1(newline, many1(parse_tile)),
        |map| -> Result<_, String> {
            let walls = iter_2d(&map)
                .filter(|&(_, &tile)| tile == ParseTile::Wall)
                .map(|(pos, _)| pos)
                .collect();

            let start = iter_2d(&map)
                .filter(|&(_, &tile)| tile == ParseTile::Start)
                .map(|(pos, _)| pos)
                .next()
                .ok_or("Start not found")?;

            let exit = iter_2d(&map)
                .filter(|&(_, &tile)| tile == ParseTile::Exit)
                .map(|(pos, _)| pos)
                .next()
                .ok_or("Exit not found")?;

            Ok(Game::new(walls, start, exit))
        },
    )(input)
}

fn parse_tile(input: &str) -> IResult<&str, ParseTile> {
    alt((
        map(tag("#"), |_| ParseTile::Wall),
        map(tag("E"), |_| ParseTile::Exit),
        map(tag("S"), |_| ParseTile::Start),
        map(tag("."), |_| ParseTile::Empty),
    ))(input)
}
