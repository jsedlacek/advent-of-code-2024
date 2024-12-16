use std::ops::{Add, AddAssign, Sub, SubAssign};

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
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    #[allow(dead_code)]

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

pub fn iter_2d<T, R, C>(map: R) -> impl Iterator<Item = (Point, T)>
where
    R: IntoIterator<Item = C>,
    C: IntoIterator<Item = T>,
{
    map.into_iter().enumerate().flat_map(move |(y, row)| {
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

        assert_eq!(d.rotate_right(), Direction::Right);
        assert_eq!(d.rotate_left(), Direction::Left);
        assert_eq!(d.opposite(), Direction::Down);
    }
}
