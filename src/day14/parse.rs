use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, space0},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

use crate::util::Point;

use super::robot::Robot;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list0(many1(newline), parse_robot)(input)
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    // p=0,4 v=3,-3

    map(
        tuple((tag("p="), parse_point, space0, tag("v="), parse_point)),
        |(_, p, _, _, v)| Robot::new(p, v),
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    // 0,4

    map(tuple((i64, tag(","), i64)), |(x, _, y)| Point(x, y))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_robot() {
        let res = parse_robot("p=0,4 v=3,-3");
        assert!(res.is_ok());

        let (_, robot) = res.unwrap();
        assert_eq!(robot.position, Point(0, 4));
        assert_eq!(robot.velocity, Point(3, -3));
    }

    #[test]
    fn test_parse_point() {
        let res = parse_point("0,4");
        assert!(res.is_ok());

        let (_, point) = res.unwrap();
        assert_eq!(point, Point(0, 4));

        let res = parse_point("1,-3");
        assert!(res.is_ok());

        let (_, point) = res.unwrap();
        assert_eq!(point, Point(1, -3));
    }

    #[test]
    fn test_parse_input() {
        let res = parse_input(TEST_INPUT);
        assert!(res.is_ok());

        let (_, robots) = res.unwrap();
        assert_eq!(robots[0].position, Point(0, 4));
        assert_eq!(robots[0].velocity, Point(3, -3));
    }
}
