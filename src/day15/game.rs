use std::collections::HashSet;

use crate::util::{Direction, Point};

pub struct Game {
    pub boxes: HashSet<Point>,
    pub walls: HashSet<Point>,
    pub robot: Point,
    pub instructions: Vec<Direction>,
}

impl Game {
    pub fn new(
        boxes: HashSet<Point>,
        walls: HashSet<Point>,
        robot: Point,
        instructions: Vec<Direction>,
    ) -> Game {
        Self {
            boxes,
            walls,
            robot,
            instructions,
        }
    }

    pub fn play(&mut self) -> u64 {
        for &direction in self.instructions.clone().iter() {
            let next_position = self.robot + direction;

            if self.walls.contains(&next_position) {
                continue;
            }

            if self.boxes.contains(&next_position) {
                if !self.push_box(next_position, direction) {
                    continue;
                }
            }

            self.robot = self.robot + direction;
        }

        self.boxes.iter().map(|p| p.0 + p.1 * 100).sum::<i64>() as u64
    }

    fn push_box(&mut self, point: Point, direction: Direction) -> bool {
        if !self.boxes.contains(&point) {
            return true;
        }

        if self.walls.contains(&(point + direction)) {
            return false;
        }

        if !self.push_box(point + direction, direction) {
            return false;
        }

        self.boxes.remove(&point);
        self.boxes.insert(point + direction);

        true
    }
}
