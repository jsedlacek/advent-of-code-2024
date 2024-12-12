use std::collections::HashMap;

use nom::{
    character::{
        complete::{newline, satisfy},
        is_alphabetic,
    },
    combinator::map,
    multi::{many0, separated_list0},
    IResult,
};

use crate::util::Point;

use super::tile::Tile;

pub fn parse_input(input: &str) -> IResult<&str, HashMap<Point, Tile>> {
    map(
        separated_list0(newline, many0(map(parse_one_alpha, Tile::new))),
        |lines| {
            lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, tiles)| {
                    tiles
                        .into_iter()
                        .enumerate()
                        .map(move |(x, tile)| (Point(x as i64, y as i64), tile))
                })
                .collect()
        },
    )(input)
}

fn parse_one_alpha(input: &str) -> nom::IResult<&str, char> {
    satisfy(|c| is_alphabetic(c as u8))(input)
}
