use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::util::{Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    exit: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueItem(Point, Direction, u64, Option<(Point, Direction)>);

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
        let mut back_track: HashMap<(Point, Direction), (u64, HashSet<(Point, Direction)>)> =
            HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(self.start, Direction::Right, 0, None));

        while let Some(QueueItem(pos, dir, score, prev_node)) = queue.pop() {
            let (min_score, set) = back_track
                .entry((pos, dir))
                .or_insert_with(|| (score, HashSet::new()));

            if score > *min_score {
                continue;
            }

            if let Some(prev_node) = prev_node {
                set.insert(prev_node);
            }

            for (next_pos, next_dir, next_score) in [
                (pos + dir, dir, score + 1),
                (pos, dir.rotate_counterclockwise(), score + 1000),
                (pos, dir.rotate_clockwise(), score + 1000),
            ] {
                if !self.walls.contains(&next_pos) {
                    queue.push(QueueItem(next_pos, next_dir, next_score, Some((pos, dir))));
                }
            }
        }

        let mut points = HashSet::new();
        let mut queue = VecDeque::new();

        let dirs = Direction::all().filter_map(|dir| {
            back_track
                .get(&(self.exit, dir))
                .map(|&(score, _)| (dir, score))
        });

        let min_score = dirs.clone().map(|(_, score)| score).min().unwrap();

        for dir in dirs
            .filter(|&(_, score)| score == min_score)
            .map(|(dir, _)| dir)
        {
            queue.push_back((self.exit, dir));
        }

        while let Some((pos, dir)) = queue.pop_front() {
            points.insert(pos);

            if let Some((_, set)) = back_track.get(&(pos, dir)) {
                queue.extend(set.iter());
            }
        }

        (min_score, points)
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
        queue.push(QueueItem(Point(0, 0), Direction::Right, 0, None));
        queue.push(QueueItem(Point(1, 0), Direction::Right, 2, None));
        queue.push(QueueItem(Point(2, 0), Direction::Right, 1, None));
        queue.push(QueueItem(Point(3, 0), Direction::Right, 1, None));

        assert_eq!(queue.pop().unwrap().2, 0);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 1);
        assert_eq!(queue.pop().unwrap().2, 2);
        assert_eq!(queue.pop(), None);
    }
}
