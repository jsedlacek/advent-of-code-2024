use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, i64, newline, space0},
    combinator::{map, map_res},
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

use crate::util::Point;

use super::machine::Machine;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list0(many1(newline), parse_machine)(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map_res(
        tuple((separated_list0(newline, parse_button), newline, parse_price)),
        |(buttons, _, price)| -> Result<_, String> {
            let (_, button_a) = buttons
                .iter()
                .find(|(name, _)| *name == "A")
                .ok_or("Button A not found")?;
            let (_, button_b) = buttons
                .iter()
                .find(|(name, _)| *name == "B")
                .ok_or("Button B not found")?;

            Ok(Machine::new(*button_a, *button_b, price))
        },
    )(input)
}

fn parse_button(input: &str) -> IResult<&str, (&str, Point)> {
    // "Button A: X+94, Y+34"
    map(
        tuple((
            tag("Button "),
            alpha1,
            tag(": "),
            space0,
            tag("X"),
            i64,
            tag(","),
            space0,
            tag("Y"),
            i64,
        )),
        |(_, name, _, _, _, x, _, _, _, y)| (name, Point(x, y)),
    )(input)
}

fn parse_price(input: &str) -> IResult<&str, Point> {
    // "Prize: X=8400, Y=5400"
    map(
        tuple((
            tag("Prize: "),
            space0,
            tag("X="),
            i64,
            tag(","),
            space0,
            tag("Y="),
            i64,
        )),
        |(_, _, _, x, _, _, _, y)| Point(x, y),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_machine() {
        let res =
            parse_machine("Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400");
        assert!(res.is_ok());
        let (_, machine) = res.unwrap();
        assert_eq!(
            machine,
            Machine::new(Point(94, 34), Point(22, 67), Point(8400, 5400))
        );
    }

    #[test]
    fn test_parse_button() {
        let res = parse_button("Button A: X+94, Y+34");
        assert!(res.is_ok());
        let (_, (name, point)) = res.unwrap();
        assert_eq!(name, "A");
        assert_eq!(point, Point(94, 34));
    }

    #[test]
    fn test_parse_price() {
        let res = parse_price("Prize: X=8400, Y=5400");
        assert!(res.is_ok());
        let (_, point) = res.unwrap();
        assert_eq!(point, Point(8400, 5400));
    }

    #[test]
    fn test_parse_input() {
        let res = parse_input(TEST_INPUT);
        assert!(res.is_ok());
        let (_, machines) = res.unwrap();
        assert_eq!(
            machines[0],
            Machine::new(Point(94, 34), Point(22, 67), Point(8400, 5400))
        );
    }
}
