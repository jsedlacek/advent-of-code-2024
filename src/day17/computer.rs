use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, u64},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc,
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl Instruction {
    fn parse(opcode: u64, operand: u64) -> Self {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Clone)]
pub struct Computer {
    registers: [u64; 3],
    program: Vec<u64>,
    instruction_pointer: u64,
    output: Vec<u64>,
}

impl Computer {
    pub fn new(registers: [u64; 3], program: Vec<u64>) -> Self {
        Self {
            registers,
            program,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    pub fn clone_with_register_a(&self, register_a: u64) -> Self {
        Self {
            registers: [register_a, self.registers[1], self.registers[2]],
            program: self.program.clone(),
            instruction_pointer: self.instruction_pointer,
            output: self.output.clone(),
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                delimited(tuple((tag("Register A:"), space0)), u64, newline),
                delimited(tuple((tag("Register B:"), space0)), u64, newline),
                delimited(tuple((tag("Register C:"), space0)), u64, newline),
                many0(newline),
                preceded(
                    tuple((tag("Program:"), space0)),
                    separated_list0(tag(","), u64),
                ),
            )),
            |(a, b, c, _, program)| Self::new([a, b, c], program),
        )(input)
    }

    pub fn run(&mut self) -> Vec<u64> {
        while let (Some(&opcode), Some(&operand)) = (
            self.program.get(self.instruction_pointer as usize),
            self.program.get((self.instruction_pointer + 1) as usize),
        ) {
            let instruction = Instruction::parse(opcode, operand);

            self.run_instruction(instruction);
        }

        self.output.clone()
    }

    pub fn get_program(&self) -> Vec<u64> {
        self.program.clone()
    }

    fn get_combo_value(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[(operand - 4) as usize],
            _ => panic!("Invalid combo value"),
        }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        let mut jump = false;

        match instruction {
            Instruction::Adv(operand) => {
                self.registers[0] /= 2u64.pow(self.get_combo_value(operand) as u32)
            }
            Instruction::Bxl(operand) => self.registers[1] ^= operand,
            Instruction::Bst(operand) => self.registers[1] = self.get_combo_value(operand) % 8,
            Instruction::Jnz(operand) => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = operand;
                    jump = true;
                }
            }
            Instruction::Bxc => self.registers[1] ^= self.registers[2],
            Instruction::Out(operand) => self.output(self.get_combo_value(operand) % 8),
            Instruction::Bdv(operand) => {
                self.registers[1] =
                    self.registers[0] / 2u64.pow(self.get_combo_value(operand) as u32)
            }
            Instruction::Cdv(operand) => {
                self.registers[2] =
                    self.registers[0] / 2u64.pow(self.get_combo_value(operand) as u32)
            }
        }

        if !jump {
            self.instruction_pointer += 2;
        }
    }

    fn output(&mut self, operand: u64) {
        self.output.push(operand);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_computer() {
        let mut computer = Computer::new([0, 0, 9], vec![2, 6]);
        computer.run();
        assert_eq!(computer.registers[1], 1);

        let mut computer = Computer::new([10, 0, 0], vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(computer.run(), vec![0, 1, 2]);

        let mut computer = Computer::new([2024, 0, 0], vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(computer.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.registers[0], 0);

        let mut computer = Computer::new([0, 29, 0], vec![1, 7]);
        computer.run();
        assert_eq!(computer.registers[1], 26);

        let mut computer = Computer::new([0, 2024, 43690], vec![4, 0]);
        computer.run();
        assert_eq!(computer.registers[1], 44354);
    }

    #[test]
    fn test_computer_input() {
        let (_, mut computer) = Computer::parse(TEST_INPUT).unwrap();

        assert_eq!(computer.run(), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
