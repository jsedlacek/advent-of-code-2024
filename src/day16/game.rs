use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    exit: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueItem(Point, Direction, u64, HashSet<Point>);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2).reverse()
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    pub fn new(walls: HashSet<Point>, player: Point, exit: Point) -> Self {
        Self {
            walls,
            start: player,
            exit,
        }
    }

    pub fn part1(&self) -> u64 {
        let (score, _) = self.best_paths();
        score
    }

    pub fn part2(&self) -> u64 {
        let (_, points) = self.best_paths();

        points.len() as u64
    }

    pub fn best_paths(&self) -> (u64, HashSet<Point>) {
        let mut visited = HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(self.start, Direction::Right, 0, HashSet::new()));

        let mut min_score = None;
        let mut best_path_positions = HashSet::new();

        loop {
            let QueueItem(pos, dir, score, path) = queue.pop().expect("No path found");

            if let Some(&visited_score) = visited.get(&(pos, dir)) {
                if visited_score < score {
                    continue;
                }
            }

            visited.insert((pos, dir), score);

            if let Some(min_score) = min_score {
                if score > min_score {
                    break;
                }
            }

            let mut path = path.clone();
            path.insert(pos);

            if pos == self.exit {
                if min_score.is_none() {
                    min_score = Some(score);
                }

                for &point in &path {
                    best_path_positions.insert(point);
                }
            }

            for (next_pos, next_dir, next_score) in [
                (pos + dir, dir, score + 1),
                (pos, dir.rotate_counterclockwise(), score + 1000),
                (pos, dir.rotate_clockwise(), score + 1000),
            ] {
                if !self.walls.contains(&next_pos) {
                    queue.push(QueueItem(next_pos, next_dir, next_score, path.clone()));
                }
            }
        }

        return (min_score.unwrap(), best_path_positions);
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::parse::parse_input;

    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");

    #[test]
    fn test_part1() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(game.part1(), 7036);

        let (_, game) = parse_input(TEST_INPUT_2).unwrap();

        assert_eq!(game.part1(), 11048);
    }

    #[test]
    fn test_part2() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(game.part2(), 45);

        let (_, game) = parse_input(TEST_INPUT_2).unwrap();

        assert_eq!(game.part2(), 64);
    }

    #[test]
    fn test_binary_heap() {
        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(Point(0, 0), Direction::Right, 0, HashSet::new()));
        queue.push(QueueItem(Point(1, 0), Direction::Right, 2, HashSet::new()));
        queue.push(QueueItem(Point(2, 0), Direction::Right, 1, HashSet::new()));
        queue.push(QueueItem(Point(3, 0), Direction::Right, 1, HashSet::new()));

        assert_eq!(queue.pop().unwrap().2, 0);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 2);
        assert_eq!(queue.pop(), None);
    }
}
