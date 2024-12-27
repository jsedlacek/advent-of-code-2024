mod parse;

use std::collections::{HashMap, HashSet};

use crate::Puzzle;

const INPUT: &str = include_str!("input.txt");

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

fn part1(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let (_, (values, operations)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let machine = Machine::new(operations);

    let mut result_wires = machine.get_out_wires().iter().cloned().collect::<Vec<_>>();

    result_wires.sort();

    let mut res = 0;

    for w in result_wires.iter().rev() {
        res = res * 2 + machine.get_value(&values, w).unwrap() as u64;
    }

    Ok(res)
}

fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let add_machine = Machine::new_add_machine();

    let (_, (_, bad_ops)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let mut swapped_machine = Machine::new(bad_ops);

    let mut fixes = Vec::new();

    for i in 0..=44 {
        let out_wire = format!("z{i:02}");

        let swapped_def = swapped_machine.get_operation_def(&out_wire);
        let add_def = add_machine.get_operation_def(&out_wire);

        if swapped_def != add_def {
            let fixed_op = find_fix(&swapped_machine, &add_machine, &out_wire, &out_wire);

            if let Some((a, b)) = fixed_op {
                swapped_machine.fix_operation((&a, &b));

                fixes.extend([a, b]);
            } else {
                panic!("Invalid wire {out_wire} = {} != {}", &swapped_def, &add_def);
            }
        }
    }

    fixes.sort();

    Ok(fixes.join(","))
}

struct Machine {
    ops: HashMap<String, Operation>,
}

impl Machine {
    fn new(ops: HashMap<String, Operation>) -> Self {
        Self { ops }
    }

    fn new_add_machine() -> Self {
        let mut ops = HashMap::new();

        for i in 0..=44 {
            ops.insert(format!("carry{i:02}"), {
                let i = i;
                // c(i)=(x(i)∧y(i))∨(c(i−1)∧(x(i)⊕y(i)))

                if i == 0 {
                    Operation::new(Operator::And, format!("x{i:02}"), format!("y{i:02}"))
                } else {
                    Operation::new(
                        Operator::Or,
                        format!("and_xy{:02}", i),
                        format!("carry_prop{:02}", i),
                    )
                }
            });
            ops.insert(format!("z{i:02}"), {
                let i = i;
                // s(i)=x(i)⊕y(i)⊕c(i−1)

                if i == 0 {
                    Operation::new(Operator::Xor, format!("x{i:02}"), format!("y{i:02}"))
                } else {
                    Operation::new(
                        Operator::Xor,
                        format!("xor_xy{:02}", i),
                        format!("carry{:02}", i - 1),
                    )
                }
            });
            ops.insert(format!("xor_xy{i:02}"), {
                let i = i;
                Operation::new(Operator::Xor, format!("x{i:02}"), format!("y{i:02}"))
            });
            ops.insert(format!("and_xy{i:02}"), {
                let i = i;
                Operation::new(Operator::And, format!("x{i:02}"), format!("y{i:02}"))
            });

            if i > 0 {
                ops.insert(format!("carry_prop{i:02}"), {
                    let i = i;
                    Operation::new(
                        Operator::And,
                        format!("carry{:02}", i - 1),
                        format!("xor_xy{:02}", i),
                    )
                });
            }
        }

        Self { ops }
    }

    fn get_out_wires(&self) -> HashSet<String> {
        self.ops
            .keys()
            .filter(|var| var.starts_with("z"))
            .cloned()
            .collect()
    }

    fn get_value(&self, values: &HashMap<String, bool>, name: &str) -> Option<bool> {
        if let Some(&value) = values.get(name) {
            return Some(value);
        }

        if let Some(operation) = self.ops.get(name) {
            return Some(operation.op.calc(
                self.get_value(values, &operation.in_a)?,
                self.get_value(values, &operation.in_b)?,
            ));
        }

        None
    }

    fn get_operation_def(&self, name: &str) -> String {
        if let Some(operation) = self.ops.get(name) {
            let (mut a, mut b) = (
                self.get_operation_def(&operation.in_a),
                self.get_operation_def(&operation.in_b),
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

    fn fix_operation(&mut self, fix: (&str, &str)) {
        let (a, b) = fix;

        self.ops = self
            .ops
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
    }
}

fn find_fix(
    bad: &Machine,
    good: &Machine,
    bad_name: &str,
    good_name: &str,
) -> Option<(String, String)> {
    let def = good.get_operation_def(good_name);

    let fixed_op = bad
        .ops
        .iter()
        .find(|(name, _)| bad.get_operation_def(name) == def);

    if let Some((fixed_op, _)) = fixed_op {
        return Some((fixed_op.to_string(), bad_name.to_string()));
    }

    if let (Some(bad_op), Some(good_op)) = (bad.ops.get(bad_name), good.ops.get(bad_name)) {
        if bad_op.op == good_op.op {
            let bad_def_a = bad.get_operation_def(&bad_op.in_a);
            let bad_def_b = bad.get_operation_def(&bad_op.in_b);
            let good_def_a = good.get_operation_def(&good_op.in_a);
            let good_def_b = good.get_operation_def(&good_op.in_b);

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
