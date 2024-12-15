use std::collections::{HashSet, VecDeque};

use crate::util::{Direction, Point};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameBox {
    points: HashSet<Point>,
}

impl GameBox {
    pub fn new(points: HashSet<Point>) -> GameBox {
        Self { points }
    }

    fn move_direction(&mut self, direction: Direction) {
        self.points = self.points.iter().map(|&p| p + direction).collect();
    }

    fn pos(&self) -> Point {
        *self.points.iter().min_by_key(|Point(x, y)| (x, y)).unwrap()
    }

    fn expand(&self) -> GameBox {
        GameBox::new(Game::expand_points_set(&self.points))
    }
}

pub struct Game {
    boxes: Vec<GameBox>,
    walls: HashSet<Point>,
    pub robot: Point,
    instructions: Vec<Direction>,
}

impl Game {
    pub fn new(
        boxes: Vec<GameBox>,
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

    fn expand_point(point: Point) -> Point {
        Point(point.0 * 2, point.1)
    }

    fn expand_points_set(points: &HashSet<Point>) -> HashSet<Point> {
        points
            .into_iter()
            .flat_map(|&p| {
                let p = Self::expand_point(p);
                [p, p + Direction::Right]
            })
            .collect()
    }

    pub fn expand(&self) -> Self {
        Self {
            boxes: self.boxes.iter().map(|b| b.expand()).collect(),
            walls: self
                .walls
                .iter()
                .flat_map(|&p| Self::expand_points_set(&HashSet::from([p])))
                .collect(),
            robot: Self::expand_point(self.robot),
            instructions: self.instructions.clone(),
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
