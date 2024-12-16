use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use super::tile::Tile;
use crate::util::{Direction, Point};

pub struct Region {
    points: HashSet<Point>,
    tile: Tile,
}

impl Region {
    pub fn find_regions(map: &HashMap<Point, Tile>) -> Vec<Region> {
        let mut processed_points = HashSet::new();
        let mut regions = Vec::new();

        for (&point, &tile) in map.iter() {
            if processed_points.contains(&point) {
                continue;
            }

            let mut region_points = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(point);

            while let Some(current_point) = queue.pop_front() {
                if processed_points.contains(&current_point) {
                    continue;
                }

                region_points.insert(current_point);
                processed_points.insert(current_point);

                for direction in Direction::all() {
                    let neighbor_point = current_point + direction;

                    if map.get(&neighbor_point) == Some(&tile)
                        && !processed_points.contains(&neighbor_point)
                    {
                        queue.push_back(neighbor_point);
                    }
                }
            }

            regions.push(Self {
                tile,
                points: region_points,
            });
        }

        regions
    }

    fn area(&self) -> u64 {
        self.points.len() as u64
    }

    fn perimeter(&self) -> u64 {
        self.points
            .iter()
            .map(|&point| {
                Direction::all()
                    .into_iter()
                    .filter(|&direction| !self.points.contains(&(point + direction)))
                    .count() as u64
            })
            .sum()
    }

    fn sides(&self) -> u64 {
        let mut points_vec = Vec::from_iter(self.points.iter().copied());
        points_vec.sort();

        let mut sides = 0;
        let mut processed_edges = HashSet::new();

        for point in points_vec {
            for direction in Direction::all() {
                if self.points.contains(&(point + direction)) {
                    continue;
                }

                let left_neighbor = point + direction.rotate_counterclockwise();
                let right_neighbor = point + direction.rotate_clockwise();

                if !processed_edges.contains(&(left_neighbor, direction))
                    && !processed_edges.contains(&(right_neighbor, direction))
                {
                    sides += 1;
                }

                processed_edges.insert((point, direction));
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

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Region {}", self.tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0)])
            }
            .area(),
            1
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1)])
            }
            .area(),
            2
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .area(),
            3
        );
    }

    #[test]
    fn test_perimeter() {
        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0)])
            }
            .perimeter(),
            4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1)])
            }
            .perimeter(),
            6
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .perimeter(),
            8
        );
    }

    #[test]
    fn test_sides() {
        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0)])
            }
            .sides(),
            4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1)])
            }
            .sides(),
            4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .sides(),
            6
        );
    }

    #[test]
    fn test_region_price_v1() {
        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0)])
            }
            .price_v1(),
            1 * 4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1)])
            }
            .price_v1(),
            2 * 6
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .price_v1(),
            3 * 8
        );
    }

    #[test]
    fn test_region_price_v2() {
        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0)])
            }
            .price_v2(),
            1 * 4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1)])
            }
            .price_v2(),
            2 * 4
        );

        assert_eq!(
            Region {
                tile: Tile::new('A'),
                points: HashSet::from([Point(0, 0), Point(0, 1), Point(1, 1)])
            }
            .price_v2(),
            3 * 6
        );
    }
}
