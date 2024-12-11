use std::error::Error;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Stone(u64);

impl Stone {
    pub fn new(stone: u64) -> Self {
        Self(stone)
    }

    pub fn transform(&self) -> Result<Vec<Stone>, Box<dyn Error>> {
        if self.0 == 0 {
            return Ok(vec![Stone(1)]);
        }

        let stone_str = self.to_string();

        if stone_str.len() % 2 == 0 {
            let (first, second) = stone_str.split_at(stone_str.len() / 2);
            Ok(vec![Stone(first.parse()?), Stone(second.parse()?)])
        } else {
            Ok(vec![self.checked_mul(2024).ok_or("Overflow")?])
        }
    }

    fn checked_mul(&self, rhs: u64) -> Option<Self> {
        self.0.checked_mul(rhs).map(Stone)
    }
}

impl ToString for Stone {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(Stone(0).transform().unwrap(), vec![Stone(1)]);
        assert_eq!(Stone(1).transform().unwrap(), vec![Stone(2024)]);
        assert_eq!(Stone(2024).transform().unwrap(), vec![Stone(20), Stone(24)]);
    }
}
