use std::{
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
