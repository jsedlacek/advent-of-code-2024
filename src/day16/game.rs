use std::collections::{BinaryHeap, HashSet};

use crate::util::{Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    exit: Point,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct QueueItem(Point, Direction, u64);

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

    pub fn play(&self) -> u64 {
        let mut visited = HashSet::new();

        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(self.start, Direction::Right, 0));

        loop {
            let QueueItem(pos, dir, score) = queue.pop().expect("No path found");

            if visited.contains(&(pos, dir)) {
                continue;
            }

            visited.insert((pos, dir));

            if pos == self.exit {
                return score;
            }

            for (next_pos, next_dir, next_score) in [
                (pos + dir, dir, score + 1),
                (pos, dir.rotate_counterclockwise(), score + 1000),
                (pos, dir.rotate_clockwise(), score + 1000),
            ] {
                if !self.walls.contains(&next_pos) {
                    queue.push(QueueItem(next_pos, next_dir, next_score));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::parse::parse_input;

    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");
    const TEST_INPUT_2: &str = include_str!("test-input-2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_play() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();

        assert_eq!(game.play(), 7036);

        let (_, game) = parse_input(TEST_INPUT_2).unwrap();

        assert_eq!(game.play(), 11048);

        let (_, game) = parse_input(INPUT).unwrap();

        assert_eq!(game.play(), 134588);
    }

    #[test]
    fn test_binary_heap() {
        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(Point(0, 0), Direction::Right, 0));
        queue.push(QueueItem(Point(1, 0), Direction::Right, 2));
        queue.push(QueueItem(Point(2, 0), Direction::Right, 1));
        queue.push(QueueItem(Point(3, 0), Direction::Right, 1));

        assert_eq!(queue.pop().unwrap().2, 0);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 2);
        assert_eq!(queue.pop(), None);
    }
}
