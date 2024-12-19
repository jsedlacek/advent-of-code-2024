use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space0},
    multi::{many1, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    (separated_pair(
        separated_list0(tag(", "), delimited(space0::<&str, _>, alpha1, space0)),
        many1(newline),
        separated_list0(newline, alpha1),
    ))(input)
}
