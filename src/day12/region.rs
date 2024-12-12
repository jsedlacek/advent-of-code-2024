use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::{Direction, Point};

use super::tile::Tile;

pub struct Region {
    points: HashSet<Point>,
}

impl Region {
    pub fn find_regions(map: &HashMap<Point, Tile>) -> Vec<Region> {
        let mut processed = HashSet::new();
        let mut regions = Vec::new();

        for (&pos, item) in map.iter() {
            if processed.contains(&pos) {
                continue;
            }

            let mut points = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(pos);

            while let Some(queue_pos) = queue.pop_front() {
                if processed.contains(&queue_pos) {
                    continue;
                }

                points.insert(queue_pos);
                processed.insert(queue_pos);

                for dir in Direction::all() {
                    let neighbot_pos = queue_pos + dir;

                    if map.get(&neighbot_pos) != Some(item) {
                        continue;
                    }

                    if processed.contains(&neighbot_pos) {
                        continue;
                    }

                    queue.push_back(neighbot_pos);
                }
            }

            regions.push(Region { points });
        }

        regions
    }

    fn area(&self) -> u64 {
        self.points.len() as u64
    }

    fn perimeter(&self) -> u64 {
        self.points
            .iter()
            .map(|&pos| {
                Direction::all()
                    .into_iter()
                    .filter(|&dir| !self.points.contains(&(pos + dir)))
                    .count() as u64
            })
            .sum::<u64>()
    }

    fn sides(&self) -> u64 {
        let mut points_vec = self.points.iter().cloned().collect::<Vec<_>>();
        points_vec.sort();

        let mut processed = HashSet::new();
        let mut sides = 0;

        for pos in points_vec {
            for dir in Direction::all() {
                if self.points.contains(&(pos + dir)) {
                    continue;
                }

                if [pos + dir.rotate_left(), pos + dir.rotate_right()]
                    .iter()
                    .all(|&p| !processed.contains(&(p, dir)))
                {
                    sides += 1;
                }

                processed.insert((pos, dir));
            }
        }

        sides
    }

    pub fn price_v1(&self) -> u64 {
        self.area() * self.perimeter()
    }

    pub fn price_v2(&self) -> u64 {
        self.area() * self.sides()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0)])
            }
            .area(),
            1
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1)])
            }
            .area(),
            2
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .area(),
            3
        );
    }

    #[test]
    fn test_perimeter() {
        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0)])
            }
            .perimeter(),
            4
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1)])
            }
            .perimeter(),
            6
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .perimeter(),
            8
        );
    }

    #[test]
    fn test_sides() {
        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0)])
            }
            .sides(),
            4
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1)])
            }
            .sides(),
            4
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .sides(),
            6
        );
    }

    #[test]
    fn test_region_price_v1() {
        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0)])
            }
            .price_v1(),
            (1 * 4)
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1)])
            }
            .price_v1(),
            (2 * 6)
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .price_v1(),
            (3 * 8)
        );
    }

    #[test]
    fn test_region_price_v2() {
        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0)])
            }
            .price_v2(),
            (1 * 4)
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1)])
            }
            .price_v2(),
            (2 * 4)
        );

        assert_eq!(
            Region {
                points: HashSet::from_iter([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .price_v2(),
            (3 * 6)
        );
    }
}
