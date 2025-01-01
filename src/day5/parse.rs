use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

use super::{Rule, RuleSet, Update};

pub fn parse_input(input: &str) -> IResult<&str, (RuleSet, Vec<Update>)> {
    terminated(
        separated_pair(
            map(separated_list1(multispace1, parse_rule), |list| {
                RuleSet::new(&list)
            }),
            multispace1,
            separated_list1(multispace1, parse_update),
        ),
        multispace0,
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(separated_pair(u64, tag("|"), u64), |pair| {
        Rule(pair.0, pair.1)
    })(input)
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    map(separated_list1(tag(","), u64), Update)(input)
}

#[cfg(test)]
mod tests {
    use crate::day5::Rule;

    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_rule() {
        let result = parse_rule("199|200");
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, Rule(199, 200));
    }

    #[test]
    fn test_parse_update() {
        let result = parse_update("75,29,13");
        assert!(result.is_ok());
        let (remaining, result) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(result, Update(vec![75, 29, 13]));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok());
        let (remaining, (rule_set, updates)) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(rule_set.precedence.len(), 21);
        assert!(rule_set.precedence.contains(&(47, 53)));
        assert!(rule_set.precedence.contains(&(53, 13)));
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], Update(vec![75, 47, 61, 53, 29]));
        assert_eq!(updates[5], Update(vec![97, 13, 75, 29, 47]));
    }
}
