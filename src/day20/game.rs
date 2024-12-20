use std::collections::{HashMap, HashSet};

use crate::util::{bfs, Direction, Point};

pub struct Game {
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Game {
    pub fn new(map: HashSet<Point>, start: Point, end: Point) -> Self {
        Self {
            walls: map,
            start,
            end,
        }
    }

    fn find_shortest_path(&self) -> Option<(u64, Vec<Point>)> {
        bfs(self.start, self.end, |pos| {
            Direction::all()
                .map(move |direction| pos + direction)
                .filter(|point| !self.walls.contains(&point))
        })
    }

    pub fn find_cheat_speedups(&self) -> HashMap<u64, u64> {
        let mut paths: HashMap<u64, u64> = HashMap::new();

        let (_, original_path) = self.find_shortest_path().unwrap();

        let original_set: HashSet<Point> = HashSet::from_iter(original_path.iter().copied());

        for (cheat_start, &pos) in original_path.iter().enumerate() {
            for direction in Direction::all() {
                let cheat_pos = pos + direction;
                if !self.walls.contains(&cheat_pos) {
                    continue;
                }

                for direction in Direction::all() {
                    let path_pos = cheat_pos + direction;
                    if !original_set.contains(&path_pos) {
                        continue;
                    }

                    let cheat_end = original_path
                        .iter()
                        .enumerate()
                        .find(|(_, &p)| p == path_pos)
                        .unwrap()
                        .0;

                    if cheat_start >= cheat_end {
                        continue;
                    }

                    let speedup = (cheat_end - cheat_start - 2) as u64;

                    *paths.entry(speedup).or_insert(0) += 1;
                }
            }
        }

        paths
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::parse::parse_input;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_find_shortest_path() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();
        let (len, path) = game.find_shortest_path().unwrap();
        assert_eq!(len, 84);
        assert_eq!(path.len(), 85); // includes start and end
    }

    #[test]
    fn test_find_paths() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();

        let speedups = game.find_cheat_speedups();

        assert_eq!(speedups.get(&2), Some(&14));
        assert_eq!(speedups.get(&4), Some(&14));
        assert_eq!(speedups.get(&6), Some(&2));
        assert_eq!(speedups.get(&8), Some(&4));
        assert_eq!(speedups.get(&10), Some(&2));
        assert_eq!(speedups.get(&12), Some(&3));
        assert_eq!(speedups.get(&20), Some(&1));
        assert_eq!(speedups.get(&36), Some(&1));
        assert_eq!(speedups.get(&38), Some(&1));
        assert_eq!(speedups.get(&40), Some(&1));
        assert_eq!(speedups.get(&64), Some(&1));
    }
}
