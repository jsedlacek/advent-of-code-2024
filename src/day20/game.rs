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

    pub fn find_cheat_speedups(&self, max_cheat_len: u64) -> HashMap<u64, u64> {
        let mut speedups = HashMap::new();

        if let Some((_, original_path)) = self.find_shortest_path() {
            original_path
                .iter()
                .enumerate()
                .flat_map(|(start, &start_pos)| {
                    original_path
                        .iter()
                        .enumerate()
                        .skip(start + 2)
                        .map(move |(end, &end_pos)| (start, start_pos, end, end_pos))
                })
                .filter_map(|(start, start_pos, end, end_pos)| {
                    let cheat_distance = end_pos.distance(start_pos);
                    let original_distance = (end - start) as u64;

                    if cheat_distance <= max_cheat_len {
                        Some(original_distance - cheat_distance)
                    } else {
                        None
                    }
                })
                .for_each(|speedup| {
                    *speedups.entry(speedup).or_insert(0) += 1;
                });
        }

        speedups
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
    fn test_find_cheat_speedups() {
        let (_, game) = parse_input(TEST_INPUT).unwrap();

        let speedups = game.find_cheat_speedups(2);

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
