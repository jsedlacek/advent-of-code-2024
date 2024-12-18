use std::collections::HashSet;

use crate::util::{bfs, Direction, Point};

pub struct Game {
    corrupted_points: HashSet<Point>,
    size: Point,
}

impl Game {
    pub fn new(corrupted_points: HashSet<Point>, size: Point) -> Self {
        Self {
            corrupted_points,
            size,
        }
    }

    pub fn find_path(&self, start: Point, end: Point) -> Option<u64> {
        if let Some((len, _)) = bfs(start, end, |point| {
            Direction::all()
                .map(move |direction| point + direction)
                .filter(|point| {
                    if self.corrupted_points.contains(&point) {
                        false
                    } else if point.0 < 0
                        || point.1 < 0
                        || point.0 >= self.size.0
                        || point.1 >= self.size.1
                    {
                        false
                    } else {
                        true
                    }
                })
        }) {
            return Some(len);
        }

        None
    }
}
