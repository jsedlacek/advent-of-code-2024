use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn wrap(self, other: Point) -> Self {
        let Point(x, y) = self;
        let Point(max_x, max_y) = other;

        Point(x.rem_euclid(max_x), y.rem_euclid(max_y))
    }

    pub fn distance(self, other: Point) -> u64 {
        let Point(x1, y1) = self;
        let Point(x2, y2) = other;

        ((x1 - x2).abs() + (y1 - y2).abs()) as u64
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, dir: Direction) -> Self::Output {
        self + match dir {
            Direction::Up => Point(0, -1),
            Direction::Right => Point(1, 0),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    #[allow(dead_code)]
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all() -> impl Iterator<Item = Direction> + Clone {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

pub fn iter_2d<T>(
    map: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
) -> impl Iterator<Item = (Point, T)> {
    map.into_iter().enumerate().flat_map(|(y, row)| {
        row.into_iter()
            .enumerate()
            .map(move |(x, value)| (Point(x as i64, y as i64), value))
    })
}

pub fn binary_search<T>(min: T, max: T, f: impl Fn(T) -> bool) -> T
where
    T: Copy
        + PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + From<u8>,
{
    let (mut l, mut r) = (min, max);

    while l < r {
        let mid = (l + r) / T::from(2u8);

        if f(mid) {
            r = mid;
        } else {
            l = mid + T::from(1u8);
        }
    }

    l - T::from(1u8)
}

pub fn bfs<T, I>(start: T, end: T, get_neighbors: impl Fn(T) -> I) -> Option<(u64, Vec<T>)>
where
    T: Eq + std::hash::Hash + Copy,
    I: Iterator<Item = T>,
{
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]); // Initialize visited with start point
    let mut came_from = HashMap::new();

    while let Some((point, distance)) = queue.pop_front() {
        if point == end {
            let mut path = vec![point];
            let mut current = point;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some((distance, path));
        }

        for neighbor in get_neighbors(point) {
            if visited.contains(&neighbor) {
                continue;
            }

            queue.push_back((neighbor, distance + 1));
            visited.insert(neighbor);
            came_from.insert(neighbor, point);
        }
    }

    None
}

#[derive(Debug, Clone)]
pub struct PointRange {
    start: Point,
    end: Point,
}

impl PointRange {
    pub fn contains(&self, point: Point) -> bool {
        let Point(x, y) = point;
        let Point(start_x, start_y) = self.start;
        let Point(end_x, end_y) = self.end;

        x >= start_x && x < end_x && y >= start_y && y < end_y
    }

    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point(11, 22);

        assert_eq!(p.wrap(Point(10, 10)), Point(1, 2));

        let p = Point(-5, -6);

        assert_eq!(p.wrap(Point(10, 10)), Point(5, 4));
    }

    #[test]
    fn test_direction() {
        let d = Direction::Up;

        assert_eq!(d.rotate_clockwise(), Direction::Right);
        assert_eq!(d.rotate_counterclockwise(), Direction::Left);
        assert_eq!(d.opposite(), Direction::Down);
    }
}
