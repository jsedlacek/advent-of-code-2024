use std::collections::HashMap;

pub struct Machine {
    ops: HashMap<String, Operation>,
}

impl Machine {
    pub fn new(ops: HashMap<String, Operation>) -> Self {
        Self { ops }
    }

    pub fn new_add_machine(size: usize) -> Self {
        let mut ops = HashMap::new();

        for i in 0..size {
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
                } else if i == size - 1 {
                    Operation::new(
                        Operator::Or,
                        format!("and_xy{:02}", i - 1),
                        format!("carry_prop{:02}", i - 1),
                    )
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

    pub fn get_out_wires(&self) -> Vec<String> {
        let mut out_wires = self
            .ops
            .keys()
            .filter(|var| var.starts_with("z"))
            .cloned()
            .collect::<Vec<_>>();

        out_wires.sort();

        out_wires
    }

    pub fn get_value(&self, values: &HashMap<String, bool>, name: &str) -> Option<bool> {
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

    pub fn get_operation_def(&self, name: &str) -> String {
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

    pub fn fix_operation(&mut self, fix: (&str, &str)) {
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

    pub fn find_fix(
        &self,
        name: &str,
        correct_machine: &Machine,
        correct_name: &str,
    ) -> Option<(String, String)> {
        let def = correct_machine.get_operation_def(correct_name);

        let fixed_op = self
            .ops
            .iter()
            .find(|(name, _)| self.get_operation_def(name) == def);

        if let Some((fixed_op, _)) = fixed_op {
            return Some((fixed_op.to_string(), name.to_string()));
        }

        if let (Some(op), Some(good_op)) = (self.ops.get(name), correct_machine.ops.get(name)) {
            if op.op == good_op.op {
                let def_a = self.get_operation_def(&op.in_a);
                let bad_def_b = self.get_operation_def(&op.in_b);
                let correct_def_a = correct_machine.get_operation_def(&good_op.in_a);
                let correct_def_b = correct_machine.get_operation_def(&good_op.in_b);

                if def_a == correct_def_a {
                    return self.find_fix(&op.in_b, correct_machine, &good_op.in_b);
                }
                if bad_def_b == correct_def_b {
                    return self.find_fix(&op.in_a, correct_machine, &good_op.in_a);
                }
                if def_a == correct_def_b {
                    return self.find_fix(&op.in_b, correct_machine, &good_op.in_a);
                }
                if bad_def_b == correct_def_a {
                    return self.find_fix(&op.in_a, correct_machine, &good_op.in_b);
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
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
pub struct Operation {
    op: Operator,
    in_a: String,
    in_b: String,
}

impl Operation {
    pub fn new(op: Operator, in_a: String, in_b: String) -> Self {
        Self { op, in_a, in_b }
    }
}
