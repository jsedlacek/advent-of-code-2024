use std::collections::HashMap;

use crate::util::Point;

use super::tile::Tile;

pub fn parse_input(input: &str) -> HashMap<Point, Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point(x as i64, y as i64), Tile::new(c)))
        })
        .collect()
}
