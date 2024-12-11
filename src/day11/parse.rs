use nom::{
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};

pub fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123 456 789";
        let expected = vec![123, 456, 789];
        assert_eq!(parse_input(input), Ok(("", expected)));
    }
}
