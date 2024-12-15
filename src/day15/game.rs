use std::collections::{HashSet, VecDeque};

use crate::util::{Direction, Point};

pub struct Game {
    boxes: Vec<GameBox>,
    wall_positions: HashSet<Point>,
    pub robot_position: Point,
    instructions: Vec<Direction>,
}

impl Game {
    pub fn new(
        boxes: Vec<GameBox>,
        wall_positions: HashSet<Point>,
        robot_position: Point,
        instructions: Vec<Direction>,
    ) -> Self {
        Self {
            boxes,
            wall_positions,
            robot_position,
            instructions,
        }
    }

    fn expand_point(point: Point) -> Point {
        Point(point.0 * 2, point.1)
    }

    fn expand_points_set(points: impl Iterator<Item = Point>) -> impl Iterator<Item = Point> {
        points.flat_map(|p| {
            let p = Self::expand_point(p);
            [p, p + Direction::Right]
        })
    }

    pub fn expand(&self) -> Self {
        Self {
            boxes: self.boxes.iter().map(|b| b.expand()).collect(),
            wall_positions: self
                .wall_positions
                .iter()
                .flat_map(|&p| Self::expand_points_set(std::iter::once(p)))
                .collect(),
            robot_position: Self::expand_point(self.robot_position),
            instructions: self.instructions.clone(),
        }
    }

    pub fn play(&mut self) -> u64 {
        for &direction in &self.instructions.clone() {
            let next_position = self.robot_position + direction;

            if self.wall_positions.contains(&next_position) {
                continue;
            }

            if !self.push_boxes(next_position, direction) {
                continue;
            }

            self.robot_position += direction;
        }

        self.boxes
            .iter()
            .filter_map(|b| b.pos())
            .map(|Point(x, y)| x + y * 100)
            .sum::<i64>() as u64
    }

    fn push_boxes(&mut self, point: Point, direction: Direction) -> bool {
        let mut source_points = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(point);

        while let Some(point) = queue.pop_front() {
            if source_points.contains(&point) {
                continue;
            }

            let box_points = self
                .boxes
                .iter()
                .filter(|b| b.points.contains(&point))
                .flat_map(|b| b.points.iter().copied());

            for p in box_points {
                source_points.insert(p);

                let target_point = p + direction;
                if !source_points.contains(&target_point) {
                    queue.push_back(target_point);
                }
            }
        }

        if source_points
            .iter()
            .map(|&p| p + direction)
            .any(|p| self.wall_positions.contains(&p))
        {
            return false;
        }

        for b in self.boxes.iter_mut() {
            if !b.points.is_disjoint(&source_points) {
                b.move_direction(direction);
            }
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameBox {
    points: HashSet<Point>,
}

impl GameBox {
    pub fn new(points: HashSet<Point>) -> Self {
        Self { points }
    }

    fn move_direction(&mut self, direction: Direction) {
        self.points = self.points.iter().map(|&p| p + direction).collect();
    }

    fn pos(&self) -> Option<Point> {
        self.points.iter().min_by_key(|Point(x, y)| (x, y)).copied()
    }

    fn expand(&self) -> Self {
        Self::new(Game::expand_points_set(self.points.iter().copied()).collect())
    }
}
