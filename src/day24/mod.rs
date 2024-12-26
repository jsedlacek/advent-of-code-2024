mod parse;

use std::collections::{HashMap, HashSet};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "AND",
                Self::Or => "OR",
                Self::Xor => "XOR",
            }
        )
    }
}

impl Operator {
    fn calc(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    op: Operator,
    in_a: String,
    in_b: String,
}

impl Operation {
    pub fn new(op: Operator, in_a: String, in_b: String) -> Self {
        Self { op, in_a, in_b }
    }
}

struct Game {
    values: HashMap<String, bool>,
    operations: HashMap<String, Operation>,
}

impl Game {
    fn wires(&self) -> HashSet<String> {
        HashSet::from_iter(self.values.keys().chain(self.operations.keys()).cloned())
    }

    fn get_value(&self, name: &str) -> Option<bool> {
        if let Some(&value) = self.values.get(name) {
            return Some(value);
        }

        if let Some(operation) = self.operations.get(name) {
            return Some(operation.op.calc(
                self.get_value(&operation.in_a)?,
                self.get_value(&operation.in_b)?,
            ));
        }

        None
    }
}

fn get_operation_def(ops: &HashMap<String, Operation>, name: &str) -> String {
    if let Some(operation) = ops.get(name) {
        let (mut a, mut b) = (
            get_operation_def(ops, &operation.in_a),
            get_operation_def(ops, &operation.in_b),
        );

        if a > b {
            (a, b) = (b, a);
        }

        let def = format!("({} {} {})", a, operation.op, b);

        def
    } else {
        name.to_string()
    }
}

fn xor(a: &str, b: &str) -> Operation {
    Operation::new(Operator::Xor, a.to_string(), b.to_string())
}

fn or(a: &str, b: &str) -> Operation {
    Operation::new(Operator::Or, a.to_string(), b.to_string())
}

fn sum_bit(i: u64) -> Operation {
    // s(i)=x(i)⊕y(i)⊕c(i−1)

    if i == 0 {
        xor(&format!("x{i:02}"), &format!("y{i:02}"))
    } else {
        xor(&format!("xor_xy{:02}", i), &format!("carry{:02}", i - 1))
    }
}

fn carry_bit(i: u64) -> Operation {
    // c(i)=(x(i)∧y(i))∨(c(i−1)∧(x(i)⊕y(i)))

    if i == 0 {
        and_xy(i)
    } else {
        or(&format!("and_xy{:02}", i), &format!("carry_prop{:02}", i))
    }
}

fn carry_prop(i: u64) -> Operation {
    Operation::new(
        Operator::And,
        format!("carry{:02}", i - 1),
        format!("xor_xy{:02}", i),
    )
}

fn and_xy(i: u64) -> Operation {
    Operation::new(Operator::And, format!("x{i:02}"), format!("y{i:02}"))
}

fn xor_xy(i: u64) -> Operation {
    Operation::new(Operator::Xor, format!("x{i:02}"), format!("y{i:02}"))
}

fn part1(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let (_, (values, operations)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let game = Game { values, operations };

    let mut result_wires = game
        .wires()
        .iter()
        .filter(|var| var.starts_with("z"))
        .cloned()
        .collect::<Vec<_>>();

    result_wires.sort();

    let mut res = 0;

    for w in result_wires.iter().rev() {
        res = res * 2 + game.get_value(w).unwrap() as u64;
    }

    Ok(res)
}

fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut good_ops = HashMap::new();

    for i in 0..=44 {
        good_ops.insert(format!("carry{i:02}"), carry_bit(i));
        good_ops.insert(format!("z{i:02}"), sum_bit(i));
        good_ops.insert(format!("xor_xy{i:02}"), xor_xy(i));
        good_ops.insert(format!("and_xy{i:02}"), and_xy(i));

        if i > 0 {
            good_ops.insert(format!("carry_prop{i:02}"), carry_prop(i));
        }
    }

    let good_machine = Machine::new(good_ops);

    let (_, (_, bad_ops)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let mut bad_machine = Machine::new(bad_ops);

    let mut fixes = Vec::new();

    for i in 0..=44 {
        let out_wire = format!("z{i:02}");

        let bad_def = get_operation_def(&bad_machine.ops_by_name, &out_wire);
        let good_def = get_operation_def(&good_machine.ops_by_name, &out_wire);

        if bad_def != good_def {
            let fixed_op = find_fix(&bad_machine, &good_machine, &out_wire, &out_wire);

            if let Some((a, b)) = fixed_op {
                bad_machine.fix_operation((&a, &b));

                fixes.extend([a, b]);
            } else {
                panic!("Invalid wire {out_wire} = {} != {}", &bad_def, &good_def);
            }
        }
    }

    fixes.sort();

    Ok(fixes.join(","))
}

struct Machine {
    ops_by_name: HashMap<String, Operation>,
    names_by_def: HashMap<String, String>,
}

impl Machine {
    fn new(ops_by_name: HashMap<String, Operation>) -> Self {
        let names_by_def = Self::get_names_by_def(&ops_by_name);

        Self {
            ops_by_name,
            names_by_def,
        }
    }

    fn get_names_by_def(ops_by_name: &HashMap<String, Operation>) -> HashMap<String, String> {
        ops_by_name
            .keys()
            .map(|name| (get_operation_def(&ops_by_name, name), name.to_string()))
            .collect()
    }

    fn fix_operation(&mut self, fix: (&str, &str)) {
        let (a, b) = fix;

        self.ops_by_name = self
            .ops_by_name
            .iter()
            .map(|(name, operation)| {
                if name == a {
                    return (b.to_string(), operation.clone());
                }

                if name == b {
                    return (a.to_string(), operation.clone());
                }

                (name.to_string(), operation.clone())
            })
            .collect();

        self.names_by_def = Self::get_names_by_def(&self.ops_by_name);
    }
}

fn find_fix(
    bad: &Machine,
    good: &Machine,
    bad_name: &str,
    good_name: &str,
) -> Option<(String, String)> {
    let def = get_operation_def(&good.ops_by_name, good_name);
    let fixed_op = bad.names_by_def.get(&def);

    if let Some(fixed_op) = fixed_op {
        return Some((fixed_op.to_string(), bad_name.to_string()));
    }

    if let (Some(bad_op), Some(good_op)) = (
        bad.ops_by_name.get(bad_name),
        good.ops_by_name.get(bad_name),
    ) {
        if bad_op.op == good_op.op {
            let bad_def_a = get_operation_def(&bad.ops_by_name, &bad_op.in_a);
            let bad_def_b = get_operation_def(&bad.ops_by_name, &bad_op.in_b);
            let good_def_a = get_operation_def(&good.ops_by_name, &good_op.in_a);
            let good_def_b = get_operation_def(&good.ops_by_name, &good_op.in_b);

            if bad_def_a == good_def_a {
                return find_fix(bad, good, &bad_op.in_b, &good_op.in_b);
            }
            if bad_def_b == good_def_b {
                return find_fix(bad, good, &bad_op.in_a, &good_op.in_a);
            }
            if bad_def_a == good_def_b {
                return find_fix(bad, good, &bad_op.in_b, &good_op.in_a);
            }
            if bad_def_b == good_def_a {
                return find_fix(bad, good, &bad_op.in_a, &good_op.in_b);
            }
        }
    }

    None
}

pub struct Part1;

impl Puzzle for Part1 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        part1(INPUT).map(|res| res.to_string())
    }
}

pub struct Part2;

impl Puzzle for Part2 {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>> {
        part2(INPUT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 2024);
    }

    #[test]
    fn test_part2_input() {
        assert_eq!(part2(INPUT).unwrap(), "css,cwt,gdd,jmv,pqt,z05,z09,z37");
    }
}
