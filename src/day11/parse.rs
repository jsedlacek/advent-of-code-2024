use nom::{
    character::complete::{space1, u64},
    combinator::map,
    multi::separated_list1,
    IResult,
};

use super::game::Stone;

pub fn parse_input(input: &str) -> IResult<&str, Vec<Stone>> {
    separated_list1(space1, map(u64, Stone::new))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 456 789";
        let expected = vec![Stone::new(123), Stone::new(456), Stone::new(789)];
        assert_eq!(parse_input(input), Ok(("", expected)));
    }
}
