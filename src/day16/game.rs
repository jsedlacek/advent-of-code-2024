use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::util::{Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    exit: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node(Point, Direction);

impl Node {
    fn moves(&self) -> impl Iterator<Item = (u64, Node)> {
        let &Self(pos, dir) = self;
        return [
            (1, Node(pos + dir, dir)),
            (1000, Node(pos, dir.rotate_counterclockwise())),
            (1000, Node(pos, dir.rotate_clockwise())),
        ]
        .into_iter();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueItem(Node, u64, Option<Node>);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
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
        let mut back_track: HashMap<
            // target node
            Node,
            (
                // score
                u64,
                // previous nodes
                HashSet<Node>,
            ),
        > = HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(QueueItem(Node(self.start, Direction::Right), 0, None));

        while let Some(QueueItem(node, score, prev_node)) = queue.pop() {
            let (min_score, set) = back_track
                .entry(node)
                .or_insert_with(|| (score, HashSet::new()));

            if score > *min_score {
                continue;
            }

            if let Some(prev_node) = prev_node {
                set.insert(prev_node);
            }

            for (move_score, move_node) in node.moves() {
                if !self.walls.contains(&move_node.0) {
                    queue.push(QueueItem(move_node, score + move_score, Some(node)));
                }
            }
        }

        let mut points = HashSet::new();
        let mut queue = VecDeque::new();

        let dirs = Direction::all().filter_map(|dir| {
            back_track
                .get(&Node(self.exit, dir))
                .map(|&(score, _)| (dir, score))
        });

        let min_score = dirs.clone().map(|(_, score)| score).min().unwrap();

        for dir in dirs
            .filter(|&(_, score)| score == min_score)
            .map(|(dir, _)| dir)
        {
            queue.push_back(Node(self.exit, dir));
        }

        while let Some(node) = queue.pop_front() {
            points.insert(node.0);

            if let Some((_, set)) = back_track.get(&node) {
                queue.extend(set.iter().cloned());
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
        queue.push(QueueItem(Node(Point(0, 0), Direction::Right), 0, None));
        queue.push(QueueItem(Node(Point(1, 0), Direction::Right), 2, None));
        queue.push(QueueItem(Node(Point(2, 0), Direction::Right), 1, None));
        queue.push(QueueItem(Node(Point(3, 0), Direction::Right), 1, None));

        assert_eq!(queue.pop().unwrap().1, 0);
        assert_eq!(queue.pop().unwrap().1, 1);
        assert_eq!(queue.pop().unwrap().1, 1);
        assert_eq!(queue.pop().unwrap().1, 2);
        assert_eq!(queue.pop(), None);
    }
}
