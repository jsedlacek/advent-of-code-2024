use std::{collections::HashMap, iter::once};

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
        let points = code.iter().map(|d| d.get_position()).collect::<Vec<_>>();

        self.click_buttons(&points, 0)
    }

    pub fn get_numeric_part(digits: &[Digit]) -> u64 {
        digits
            .iter()
            .filter_map(|d| match d {
                Digit::Zero => Some(0),
                Digit::One => Some(1),
                Digit::Two => Some(2),
                Digit::Three => Some(3),
                Digit::Four => Some(4),
                Digit::Five => Some(5),
                Digit::Six => Some(6),
                Digit::Seven => Some(7),
                Digit::Eight => Some(8),
                Digit::Nine => Some(9),
                _ => None,
            })
            .fold(0, |acc, d| acc * 10 + d)
    }

    fn click_button(&mut self, current_pos: Point, target_pos: Point, level: u64) -> u64 {
        if let Some(res) = self.cache.get(&(current_pos, target_pos, level)) {
            return *res;
        }

        let diff = target_pos - current_pos;

        let x_directions = if diff.0 < 0 {
            vec![Direction::Left; diff.0.abs() as usize]
        } else if diff.0 > 0 {
            vec![Direction::Right; diff.0.abs() as usize]
        } else {
            vec![]
        };

        let y_directions = if diff.1 < 0 {
            vec![Direction::Up; diff.1.abs() as usize]
        } else if diff.1 > 0 {
            vec![Direction::Down; diff.1.abs() as usize]
        } else {
            vec![]
        };

        let mut path_a = Vec::new();
        path_a.extend(&x_directions);
        path_a.extend(&y_directions);

        let mut path_b = Vec::new();
        path_b.extend(&y_directions);
        path_b.extend(&x_directions);

        let res = [path_a, path_b]
            .into_iter()
            .filter(|path| {
                let mut pos = current_pos;
                for &dir in path {
                    pos += dir;

                    if !Self::is_position_valid(pos, level) {
                        return false;
                    }
                }

                true
            })
            .map(|path| {
                let points = path
                    .iter()
                    .map(|&d| Key::Direction(d))
                    .chain(once(Key::Activate))
                    .map(|k| k.get_position())
                    .collect::<Vec<_>>();

                self.click_buttons(&points, level + 1)
            })
            .min()
            .unwrap();

        self.cache.insert((current_pos, target_pos, level), res);

        res
    }

    fn click_buttons(&mut self, path: &[Point], level: u64) -> u64 {
        if level == self.max_level {
            return path.len() as u64;
        }

        let mut current_pos = Point(0, 0);

        let mut result = 0;
        for &p in path {
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
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'A' => Some(Self::A),
            _ => None,
        }
    }

    fn get_position(&self) -> Point {
        let digit_positions = Self::get_digit_positions();
        digit_positions
            .iter()
            .find(|(_, d)| d == self)
            .map(|(p, _)| *p)
            .unwrap()
    }

    fn get_by_pos(pos: Point) -> Option<Digit> {
        let digit_positions = Self::get_digit_positions();
        digit_positions
            .into_iter()
            .find(|(p, _)| *p == pos)
            .map(|(_, d)| d)
    }

    fn get_digit_positions() -> [(Point, Digit); 11] {
        [
            (Point(-2, -3), Digit::Seven),
            (Point(-1, -3), Digit::Eight),
            (Point(0, -3), Digit::Nine),
            (Point(-2, -2), Digit::Four),
            (Point(-1, -2), Digit::Five),
            (Point(0, -2), Digit::Six),
            (Point(-2, -1), Digit::One),
            (Point(-1, -1), Digit::Two),
            (Point(0, -1), Digit::Three),
            (Point(-1, 0), Digit::Zero),
            (Point(0, 0), Digit::A),
        ]
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Key {
    Direction(Direction),
    Activate,
}

impl Key {
    fn get_key_positions() -> [(Point, Key); 5] {
        [
            (Point(-1, 0), Key::Direction(Direction::Up)),
            (Point(-1, 1), Key::Direction(Direction::Down)),
            (Point(-2, 1), Key::Direction(Direction::Left)),
            (Point(0, 1), Key::Direction(Direction::Right)),
            (Point(0, 0), Key::Activate),
        ]
    }

    fn get_position(&self) -> Point {
        Self::get_key_positions()
            .iter()
            .find(|(_, k)| k == self)
            .map(|(p, _)| *p)
            .unwrap()
    }

    fn get_by_pos(pos: Point) -> Option<Key> {
        Self::get_key_positions()
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
        let code = [Digit::Zero, Digit::Two, Digit::Nine, Digit::A];

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
            Game::get_numeric_part(&[Digit::Zero, Digit::Two, Digit::Nine, Digit::A]),
            29
        );
    }
}
