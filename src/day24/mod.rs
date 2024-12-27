mod machine;
mod parse;

use machine::Machine;

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

fn part1(input: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let (_, (values, ops)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let machine = Machine::new(ops);

    machine
        .get_out_wires()
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let value = machine.get_value(&values, w).ok_or("Invalid wire")? as u64;
            Ok(value * 2u64.pow(i as u32))
        })
        .sum()
}

fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let add_machine = Machine::new_add_machine(46);

    let (_, (_, ops)) = parse::parse_input(input).map_err(|e| e.to_owned())?;

    let mut swapped_machine = Machine::new(ops);

    let mut fixes = Vec::new();

    for out_wire in swapped_machine.get_out_wires() {
        let swapped_def = swapped_machine.get_operation_def(&out_wire);
        let add_def = add_machine.get_operation_def(&out_wire);

        if swapped_def != add_def {
            let fixed_op = swapped_machine.find_fix(&out_wire, &add_machine, &out_wire);

            if let Some((a, b)) = fixed_op {
                swapped_machine.fix_operation((&a, &b));

                fixes.extend([a, b]);
            } else {
                Err(format!(
                    "Invalid wire {out_wire} = {} != {}",
                    &swapped_def, &add_def
                ))?;
            }
        }
    }

    fixes.sort();

    Ok(fixes.join(","))
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
