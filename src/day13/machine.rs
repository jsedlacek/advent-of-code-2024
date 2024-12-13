use crate::util::Point;

#[derive(Debug, PartialEq)]
pub struct Machine {
    button_a: Point,
    button_b: Point,
    price: Point,
}

impl Machine {
    pub fn new(button_a: Point, button_b: Point, price: Point) -> Self {
        Self {
            button_a,
            button_b,
            price,
        }
    }

    pub fn increase_prices(&mut self) {
        self.price.0 += 10000000000000;
        self.price.1 += 10000000000000;
    }

    pub fn solve(&self) -> u64 {
        let Point(bax, bay) = self.button_a;
        let Point(bbx, bby) = self.button_b;
        let Point(px, py) = self.price;

        if let Some(b) = div_without_remainder(py * bax - px * bay, bby * bax - bbx * bay) {
            if let Some(a) = div_without_remainder(px - bbx * b, bax) {
                return a as u64 * 3 + b as u64;
            }
        }

        0
    }
}

fn div_without_remainder(a: i64, b: i64) -> Option<i64> {
    if a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let machine = Machine {
            button_a: Point(94, 34),
            button_b: Point(22, 67),
            price: Point(8400, 5400),
        };

        assert_eq!(machine.solve(), 280)
    }
}
