use std::collections::HashMap;

use crate::util::{Direction, Point};

pub struct Game {
    cache: HashMap<(Point, Point, u64), u64>,
    max_level: u64,
}

impl Game {
    pub fn new(max_level: u64) -> Self {
        Self {
            cache: HashMap::new(),
            max_level,
        }
    }

    pub fn get_sequence_len(&mut self, code: &[Digit]) -> u64 {
        let points = code.iter().map(|d| d.get_position());

        self.click_buttons(points, 0)
    }

    pub fn get_numeric_part(digits: &[Digit]) -> u64 {
        digits
            .iter()
            .filter_map(|d| match d {
                Digit::Number(n) => Some(*n as u64),
                Digit::Activate => None,
            })
            .fold(0, |acc, d| acc * 10 + d)
    }

    fn click_button(&mut self, current_pos: Point, target_pos: Point, level: u64) -> u64 {
        if let Some(res) = self.cache.get(&(current_pos, target_pos, level)) {
            return *res;
        }

        let paths = self.generate_possible_paths(current_pos, target_pos, level);

        let res = paths
            .map(|path| {
                let points = path
                    .map(|d| Key::Direction(d))
                    .chain(std::iter::once(Key::Activate))
                    .map(|k| k.get_position());

                self.click_buttons(points, level + 1)
            })
            .min()
            .unwrap();

        self.cache.insert((current_pos, target_pos, level), res);

        res
    }

    fn click_buttons(&mut self, path: impl IntoIterator<Item = Point>, level: u64) -> u64 {
        if level == self.max_level {
            return path.into_iter().count() as u64;
        }

        let mut current_pos = Point(0, 0);

        let mut result = 0;
        for p in path {
            result += self.click_button(current_pos, p, level);
            current_pos = p;
        }

        result
    }

    fn is_position_valid(pos: Point, level: u64) -> bool {
        if level == 0 {
            return Digit::get_by_pos(pos).is_some();
        } else {
            return Key::get_by_pos(pos).is_some();
        }
    }

    fn generate_possible_paths(
        &self,
        current_pos: Point,
        target_pos: Point,
        level: u64,
    ) -> impl Iterator<Item = impl Iterator<Item = Direction> + Clone> {
        let diff = target_pos - current_pos;

        let (x_direction, x_len) = (
            if diff.0 < 0 {
                Direction::Left
            } else {
                Direction::Right
            },
            diff.0.abs(),
        );

        let (y_direction, y_len) = (
            if diff.1 < 0 {
                Direction::Up
            } else {
                Direction::Down
            },
            diff.1.abs(),
        );

        let x_directions = std::iter::repeat(x_direction).take(x_len as usize);
        let y_directions = std::iter::repeat(y_direction).take(y_len as usize);

        let path_a = x_directions.clone().chain(y_directions.clone());
        let path_b = y_directions.chain(x_directions);

        [path_a, path_b].into_iter().filter(move |path| {
            let mut pos = current_pos;
            for dir in path.clone() {
                pos += dir;

                if !Self::is_position_valid(pos, level) {
                    return false;
                }
            }

            true
        })
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Digit {
    Number(u8),
    Activate,
}

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0'..='9' => c.to_digit(10).map(|d| Self::Number(d as u8)),
            'A' => Some(Self::Activate),
            _ => None,
        }
    }

    fn get_position(&self) -> Point {
        Self::DIGIT_POSITIONS
            .iter()
            .find(|(_, d)| d == self)
            .map(|(p, _)| *p)
            .unwrap()
    }

    fn get_by_pos(pos: Point) -> Option<Self> {
        Self::DIGIT_POSITIONS
            .into_iter()
            .find(|(p, _)| *p == pos)
            .map(|(_, d)| d)
    }

    const DIGIT_POSITIONS: [(Point, Self); 11] = [
        (Point(-2, -3), Self::Number(7)),
        (Point(-1, -3), Self::Number(8)),
        (Point(0, -3), Self::Number(9)),
        (Point(-2, -2), Self::Number(4)),
        (Point(-1, -2), Self::Number(5)),
        (Point(0, -2), Self::Number(6)),
        (Point(-2, -1), Self::Number(1)),
        (Point(-1, -1), Self::Number(2)),
        (Point(0, -1), Self::Number(3)),
        (Point(-1, 0), Self::Number(0)),
        (Point(0, 0), Self::Activate),
    ];
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Key {
    Direction(Direction),
    Activate,
}

impl Key {
    const KEY_POSITIONS: [(Point, Self); 5] = [
        (Point(-1, 0), Self::Direction(Direction::Up)),
        (Point(-1, 1), Self::Direction(Direction::Down)),
        (Point(-2, 1), Self::Direction(Direction::Left)),
        (Point(0, 1), Self::Direction(Direction::Right)),
        (Point(0, 0), Self::Activate),
    ];

    fn get_position(&self) -> Point {
        Self::KEY_POSITIONS
            .iter()
            .find(|(_, k)| k == self)
            .map(|(p, _)| *p)
            .unwrap()
    }

    fn get_by_pos(pos: Point) -> Option<Self> {
        Self::KEY_POSITIONS
            .into_iter()
            .find(|(p, _)| *p == pos)
            .map(|(_, k)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_buttons() {
        let code = [
            Digit::Number(0),
            Digit::Number(2),
            Digit::Number(9),
            Digit::Activate,
        ];

        let mut game = Game::new(0);
        assert_eq!(game.get_sequence_len(&code), 4);

        let mut game = Game::new(1);
        assert_eq!(game.get_sequence_len(&code), 12);

        let mut game = Game::new(2);
        assert_eq!(game.get_sequence_len(&code), 28);

        let mut game = Game::new(3);
        assert_eq!(game.get_sequence_len(&code), 68);
    }

    #[test]
    fn test_get_numeric_part() {
        assert_eq!(
            Game::get_numeric_part(&[
                Digit::Number(0),
                Digit::Number(2),
                Digit::Number(9),
                Digit::Activate
            ]),
            29
        );
    }
}
