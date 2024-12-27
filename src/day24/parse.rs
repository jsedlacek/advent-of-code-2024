use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, newline, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use super::machine::{Operation, Operator};

pub fn parse_input(
    input: &str,
) -> IResult<&str, (HashMap<String, bool>, HashMap<String, Operation>)> {
    map(
        separated_pair(
            separated_list1(newline, parse_variable),
            multispace0,
            separated_list1(newline, parse_instruction),
        ),
        |(variables, instructions)| {
            (
                HashMap::from_iter(variables),
                HashMap::from_iter(instructions),
            )
        },
    )(input)
}

fn parse_variable(input: &str) -> IResult<&str, (String, bool)> {
    // x00: 1

    map(
        tuple((alphanumeric1, tag(":"), multispace1, parse_bool)),
        |(name, _, _, value)| (name.to_string(), value),
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, bool> {
    // 1

    alt((map(tag("1"), |_| true), map(tag("0"), |_| false)))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, (String, Operation)> {
    // x00 AND y00 -> z00

    map(
        tuple((
            alphanumeric1,
            space0,
            parse_operator,
            space0,
            alphanumeric1,
            space0,
            tag("->"),
            space0,
            alphanumeric1,
        )),
        |(in_a, _, operation, _, in_b, _, _, _, out)| {
            (
                out.to_string(),
                Operation::new(operation, in_a.to_string(), in_b.to_string()),
            )
        },
    )(input)
}

fn parse_operator(line: &str) -> IResult<&str, Operator> {
    // AND, OR, XOR

    alt((
        map(tag("AND"), |_| Operator::And),
        map(tag("OR"), |_| Operator::Or),
        map(tag("XOR"), |_| Operator::Xor),
    ))(line)
}
