use std::{error::Error, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Stone {
    number: u64,
}

impl Stone {
    pub fn new(number: u64) -> Self {
        Self { number }
    }

    pub fn transform(&self) -> Result<Vec<Self>, Box<dyn Error>> {
        if self.number == 0 {
            return Ok(vec![Self::new(1)]);
        }

        let number_str = self.to_string();

        if number_str.len() % 2 == 0 {
            let (first, second) = number_str.split_at(number_str.len() / 2);

            Ok([first, second]
                .iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?)
        } else {
            Ok(vec![self.checked_mul(2024).ok_or("Overflow")?])
        }
    }

    fn checked_mul(&self, rhs: u64) -> Option<Self> {
        self.number.checked_mul(rhs).map(Self::new)
    }
}

impl ToString for Stone {
    fn to_string(&self) -> String {
        self.number.to_string()
    }
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(Stone::new(0).transform().unwrap(), vec![Stone::new(1)]);

        assert_eq!(Stone::new(1).transform().unwrap(), vec![Stone::new(2024)]);

        assert_eq!(
            Stone::new(2024).transform().unwrap(),
            vec![Stone::new(20), Stone::new(24)]
        );
    }
}
