use std::collections::{HashSet, VecDeque};

use crate::util::{Direction, Point};

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
        let mut queue = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();

        while let Some((point, distance)) = queue.pop_front() {
            if point == end {
                return Some(distance);
            }

            for direction in Direction::all() {
                let neighbor = point + direction;

                if self.corrupted_points.contains(&neighbor) {
                    continue;
                }

                if neighbor.0 < 0
                    || neighbor.1 < 0
                    || neighbor.0 >= self.size.0
                    || neighbor.1 >= self.size.1
                {
                    continue;
                }

                if visited.contains(&neighbor) {
                    continue;
                }

                queue.push_back((neighbor, distance + 1));
                visited.insert(neighbor);
            }
        }

        return None;
    }
}
