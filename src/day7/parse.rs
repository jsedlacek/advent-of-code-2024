use nom::{
    bytes::streaming::tag,
    character::complete::{newline, space0, space1, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use super::Equation;

pub fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list0(newline, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    // example: "190: 10 19"
    map(
        tuple((u64, tag(":"), space0, separated_list0(space1, u64))),
        |(result, _, _, numbers)| Equation { result, numbers },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_equation() {
        let input = "123: 123 456 789";
        let expected = Equation {
            result: 123,
            numbers: vec![123, 456, 789],
        };
        assert_eq!(parse_equation(input), Ok(("", expected)));
    }
}
