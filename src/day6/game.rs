use std::collections::HashSet;

use crate::util::{Direction, Point, PointRange};

#[derive(Debug, Clone)]
pub struct Game {
    walls: HashSet<Point>,
    range: PointRange,
}

impl Game {
    pub fn new(walls: HashSet<Point>, range: PointRange) -> Game {
        Game { walls, range }
    }

    pub fn clone_with_obstacle(&self, obstacle: Point) -> Self {
        let mut game = self.clone();
        game.walls.insert(obstacle);

        game
    }

    pub fn iter(&self, guard: (Point, Direction)) -> impl Iterator<Item = (Point, Direction)> + '_ {
        let (mut pos, mut dir) = guard;

        return std::iter::once(guard).chain(std::iter::from_fn(move || loop {
            let new_pos = pos + dir;

            if !self.range.contains(new_pos) {
                return None;
            }

            if !self.walls.contains(&new_pos) {
                pos = new_pos;
                return Some((pos, dir));
            }

            dir = dir.rotate_clockwise();
        }));
    }

    pub fn is_loop(&self, guard: (Point, Direction)) -> bool {
        let mut visited_state = HashSet::new();

        for state in self.iter(guard) {
            if !visited_state.insert(state) {
                return true;
            }
        }

        false
    }
}
