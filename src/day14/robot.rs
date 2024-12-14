use crate::util::Point;

#[derive(Debug, PartialEq)]
pub struct Robot {
    pub position: Point,
    pub velocity: Point,
}

impl Robot {
    pub fn new(position: Point, velocity: Point) -> Self {
        Robot { position, velocity }
    }

    pub fn move_robot(&mut self, size: Point) {
        self.position = (self.position + self.velocity).wrap(size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot() {
        let robot = Robot {
            position: Point(0, 4),
            velocity: Point(3, -3),
        };

        assert_eq!(robot.position, Point(0, 4));
        assert_eq!(robot.velocity, Point(3, -3));
    }

    #[test]
    fn test_move() {
        let mut robot = Robot {
            position: Point(5, 6),
            velocity: Point(8, -7),
        };
        robot.move_robot(Point(10, 10));

        assert_eq!(robot.position, Point(3, 9));
    }
}
