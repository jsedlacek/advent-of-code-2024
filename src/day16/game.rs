use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::util::{Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    exit: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: Point,
    direction: Direction,
}

impl Node {
    fn moves(&self) -> impl Iterator<Item = (u64, Node)> {
        [
            (
                1,
                Node {
                    position: self.position + self.direction,
                    direction: self.direction,
                },
            ),
            (
                1000,
                Node {
                    position: self.position,
                    direction: self.direction.rotate_counterclockwise(),
                },
            ),
            (
                1000,
                Node {
                    position: self.position,
                    direction: self.direction.rotate_clockwise(),
                },
            ),
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueItem {
    node: Node,
    score: u64,
    prev_node: Option<Node>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Game {
    pub fn new(walls: HashSet<Point>, start: Point, exit: Point) -> Self {
        Self { walls, start, exit }
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
        let mut back_track: HashMap<Node, (u64, HashSet<Node>)> = HashMap::new();

        let mut queue = BinaryHeap::new();

        queue.push(QueueItem {
            node: Node {
                position: self.start,
                direction: Direction::Right,
            },
            score: 0,
            prev_node: None,
        });

        while let Some(QueueItem {
            node,
            score,
            prev_node,
        }) = queue.pop()
        {
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
                if !self.walls.contains(&move_node.position) {
                    queue.push(QueueItem {
                        node: move_node,
                        score: score + move_score,
                        prev_node: Some(node),
                    });
                }
            }
        }

        let mut points = HashSet::new();
        let mut queue = VecDeque::new();

        let dir_scores = Direction::all().filter_map(|dir| {
            back_track
                .get(&Node {
                    position: self.exit,
                    direction: dir,
                })
                .map(|&(score, _)| (dir, score))
        });

        let min_score = dir_scores.clone().map(|(_, score)| score).min().unwrap();

        queue.extend(
            dir_scores
                .filter(|&(_, score)| score == min_score)
                .map(|(dir, _)| Node {
                    position: self.exit,
                    direction: dir,
                }),
        );

        while let Some(node) = queue.pop_front() {
            points.insert(node.position);

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

        let node = Node {
            position: Point(0, 0),
            direction: Direction::Right,
        };

        queue.push(QueueItem {
            node,
            score: 0,
            prev_node: None,
        });

        queue.push(QueueItem {
            node,
            score: 2,
            prev_node: None,
        });

        queue.push(QueueItem {
            node,
            score: 1,
            prev_node: None,
        });

        queue.push(QueueItem {
            node,
            score: 1,
            prev_node: None,
        });

        assert_eq!(queue.pop().unwrap().score, 0);
        assert_eq!(queue.pop().unwrap().score, 1);
        assert_eq!(queue.pop().unwrap().score, 1);
        assert_eq!(queue.pop().unwrap().score, 2);
        assert_eq!(queue.pop(), None);
    }
}
