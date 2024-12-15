use std::collections::{HashSet, VecDeque};

use crate::util::{Direction, Point};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Box {
    points: HashSet<Point>,
}

impl Box {
    fn new(points: HashSet<Point>) -> Box {
        Self { points }
    }

    fn move_direction(&mut self, direction: Direction) {
        self.points = self.points.iter().map(|&p| p + direction).collect();
    }

    fn pos(&self) -> Point {
        *self.points.iter().min_by_key(|Point(x, y)| (x, y)).unwrap()
    }
}

#[derive(Debug, PartialEq)]
enum RobotPosition {
    Left,
    Right,
}

pub struct Game {
    boxes: Vec<Box>,
    walls: HashSet<Point>,
    pub robot: Point,
    instructions: Vec<Direction>,
}

impl Game {
    pub fn new(
        boxes: Vec<Box>,
        walls: HashSet<Point>,
        robot: Point,
        instructions: Vec<Direction>,
    ) -> Self {
        Self {
            boxes,
            walls,
            robot,
            instructions,
        }
    }

    fn point_from_v1(point: Point) -> Point {
        Point(point.0 * 2, point.1)
    }

    pub fn from_v1(game: super::game::Game) -> Self {
        Self {
            boxes: game
                .boxes
                .into_iter()
                .map(|p| {
                    let p = Self::point_from_v1(p);
                    Box::new(HashSet::from([p, p + Direction::Right]))
                })
                .collect(),
            walls: game
                .walls
                .into_iter()
                .flat_map(|p| {
                    let p = Self::point_from_v1(p);
                    [p, p + Direction::Right]
                })
                .collect(),
            robot: Self::point_from_v1(game.robot),
            instructions: game.instructions,
        }
    }

    pub fn play(&mut self) -> u64 {
        for &direction in self.instructions.clone().iter() {
            let next_position = self.robot + direction;

            if self.walls.contains(&next_position) {
                continue;
            }

            if !self.push_boxes(next_position, direction) {
                continue;
            }

            self.robot = self.robot + direction;
        }

        self.boxes
            .iter()
            .map(|b| b.pos())
            .map(|Point(x, y)| x + y * 100)
            .sum::<i64>() as u64
    }

    fn push_boxes(&mut self, point: Point, direction: Direction) -> bool {
        let mut points_to_move = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(point);

        while let Some(point) = queue.pop_front() {
            let box_points = self
                .boxes
                .iter()
                .filter(|b| b.points.contains(&point))
                .flat_map(|b| b.points.iter().copied())
                .collect::<Vec<_>>();

            for &p in box_points.iter() {
                points_to_move.insert(p);
            }

            for moved_point in box_points.iter().map(|&p| p + direction) {
                if !points_to_move.contains(&moved_point) {
                    queue.push_back(moved_point);
                }
            }
        }

        let mut moved_points = points_to_move.iter().map(|&p| p + direction);

        if moved_points.any(|p| self.walls.contains(&p)) {
            return false;
        }

        for b in self.boxes.iter_mut() {
            if b.points.is_disjoint(&points_to_move) {
                continue;
            }

            b.move_direction(direction);
        }

        true
    }
}
