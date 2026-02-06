use std::fs;

struct Literal(i32);

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
}

impl Register {
    fn from(number: usize) -> Self {
        match number {
            4 => Register::A,
            5 => Register::B,
            6 => Register::C,
            _ => panic!("Invalid register number: {}", number),
        }
    }

    fn index(&self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
        }
    }

    fn literal(&self) -> usize {
        match self {
            Register::A => 4,
            Register::B => 5,
            Register::C => 6,
        }
    }
}

#[derive(Debug)]
struct Opcode(usize);

impl Opcode {
    fn from(opcode: usize) -> Self {
        match opcode {
            0 => Self(0),
            1 => Self(1),
            2 => Self(2),
            3 => Self(3),
            4 => Self(4),
            5 => Self(5),
            6 => Self(6),
            7 => Self(7),
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }

    fn combo_operand(operand: usize, registers: &Vec<usize>) -> usize {
        if operand < 4 {
            return operand;
        }
        registers[operand - 4]
    }

    fn literal_operand(&self) -> usize {
        return self.0;
    }

    fn instruction(&self) -> fn(operand: usize) -> Instruction {
        match self.0 {
            0 => |operand: usize| {
                Instruction::Division(DivisionInstruction {
                    numerator_literal: None,
                    numerator_register: Some(Register::A),
                    denominator_literal: Some(2_usize.pow(operand as u32)),
                    denominator_register: None,
                    result_register: Some(Register::A),
                })
            },
            1 => |operand: usize| {
                Instruction::Xor(XorInstruction {
                    left_literal: None,
                    left_register: Some(Register::B),
                    right_literal: Some(operand),
                    right_register: None,
                    result_register: Some(Register::B),
                })
            },
            2 => |operand: usize| {
                Instruction::Modulo(ModuloInstruction {
                    left_literal: Some(operand),
                    left_register: None,
                    right_literal: Some(8),
                    right_register: None,
                    result_register: Some(Register::B),
                    output: false,
                    output_value: None,
                })
            },
            3 => |operand: usize| {
                Instruction::Jump(JumpInstruction {
                    to_literal: operand,
                    conditions: vec![Condition {
                        left_literal: None,
                        left_register: Some(Register::A),
                        right_literal: Some(0),
                        right_register: None,
                        operator: Operator::NotEqual,
                    }],
                })
            },
            4 => |_operand: usize| {
                Instruction::Xor(XorInstruction {
                    left_literal: None,
                    left_register: Some(Register::B),
                    right_literal: None,
                    right_register: Some(Register::C),
                    result_register: Some(Register::B),
                })
            },
            5 => |operand: usize| {
                // operand is already the resolved combo operand value
                Instruction::Modulo(ModuloInstruction {
                    left_literal: None,
                    left_register: None,
                    right_literal: Some(8),
                    right_register: None,
                    result_register: None,
                    output: true,
                    output_value: Some(operand % 8),
                })
            },
            6 => |operand: usize| {
                Instruction::Division(DivisionInstruction {
                    numerator_literal: None,
                    numerator_register: Some(Register::A),
                    denominator_literal: Some(2_usize.pow(operand as u32)),
                    denominator_register: None,
                    result_register: Some(Register::B),
                })
            },
            7 => |operand: usize| {
                Instruction::Division(DivisionInstruction {
                    numerator_literal: None,
                    numerator_register: Some(Register::A),
                    denominator_literal: Some(2_usize.pow(operand as u32)),
                    denominator_register: None,
                    result_register: Some(Register::C),
                })
            },
            _ => panic!("Invalid opcode: {}", self.0),
        }
    }
}

struct DivisionInstruction {
    numerator_literal: Option<usize>,
    denominator_literal: Option<usize>,
    numerator_register: Option<Register>,
    denominator_register: Option<Register>,
    result_register: Option<Register>,
}

impl DivisionInstruction {
    fn execute(&self, registers: &mut Vec<usize>) -> Option<usize> {
        let numerator = if let Some(lit) = self.numerator_literal {
            lit
        } else {
            registers[self.numerator_register.as_ref().unwrap().index()]
        };
        let denominator = if let Some(lit) = self.denominator_literal {
            lit
        } else {
            registers[self.denominator_register.as_ref().unwrap().index()]
        };
        let result = numerator / denominator;
        if self.result_register.is_some() {
            registers[self.result_register.as_ref().unwrap().index()] = result;
        }
        None
    }
}

struct XorInstruction {
    left_literal: Option<usize>,
    right_literal: Option<usize>,
    left_register: Option<Register>,
    right_register: Option<Register>,
    result_register: Option<Register>,
}

impl XorInstruction {
    fn execute(&self, registers: &mut Vec<usize>) -> Option<usize> {
        let left = if let Some(lit) = self.left_literal {
            lit
        } else {
            registers[self.left_register.as_ref().unwrap().index()]
        };
        let right = if let Some(lit) = self.right_literal {
            lit
        } else {
            registers[self.right_register.as_ref().unwrap().index()]
        };
        let result = left ^ right;
        if self.result_register.is_some() {
            registers[self.result_register.as_ref().unwrap().index()] = result;
        }
        None
    }
}

struct ModuloInstruction {
    left_literal: Option<usize>,
    right_literal: Option<usize>,
    left_register: Option<Register>,
    right_register: Option<Register>,
    result_register: Option<Register>,
    output: bool,
    output_value: Option<usize>,
}

impl ModuloInstruction {
    fn execute(&self, registers: &mut Vec<usize>) -> Option<usize> {
        // If this is an output instruction (opcode 5), don't modify registers
        if self.output_value.is_some() {
            return None;
        }
        let left = if let Some(lit) = self.left_literal {
            lit
        } else {
            registers[self.left_register.as_ref().unwrap().index()]
        };
        let right = if let Some(lit) = self.right_literal {
            lit
        } else {
            registers[self.right_register.as_ref().unwrap().index()]
        };
        let result = left % right;
        if self.result_register.is_some() {
            registers[self.result_register.as_ref().unwrap().index()] = result;
        }
        None
    }
}

struct Condition {
    left_literal: Option<usize>,
    left_register: Option<Register>,
    right_literal: Option<usize>,
    right_register: Option<Register>,
    operator: Operator,
}

enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

struct JumpInstruction {
    to_literal: usize,
    conditions: Vec<Condition>,
}

impl JumpInstruction {
    fn execute(&self, registers: &mut Vec<usize>) -> Option<usize> {
        for condition in &self.conditions {
            let left = if let Some(lit) = condition.left_literal {
                lit
            } else {
                registers[condition.left_register.as_ref().unwrap().index()]
            };
            let right = if let Some(lit) = condition.right_literal {
                lit
            } else {
                registers[condition.right_register.as_ref().unwrap().index()]
            };
            let result = match condition.operator {
                Operator::Equal => left == right,
                Operator::NotEqual => left != right,
                Operator::GreaterThan => left > right,
                Operator::LessThan => left < right,
                Operator::GreaterThanOrEqual => left >= right,
                Operator::LessThanOrEqual => left <= right,
            };
            if !result {
                return None;
            }
        }
        Some(self.to_literal)
    }
}

enum Instruction {
    Division(DivisionInstruction),
    Xor(XorInstruction),
    Modulo(ModuloInstruction),
    Jump(JumpInstruction),
}

impl Instruction {
    fn execute(&self, registers: &mut Vec<usize>) -> Option<usize> {
        match self {
            Instruction::Division(instruction) => instruction.execute(registers),
            Instruction::Xor(instruction) => instruction.execute(registers),
            Instruction::Modulo(instruction) => instruction.execute(registers),
            Instruction::Jump(instruction) => instruction.execute(registers),
        }
    }

    fn output(&self) -> Option<usize> {
        match self {
            Instruction::Division(_instruction) => None,
            Instruction::Xor(_instruction) => None,
            Instruction::Modulo(instruction) => {
                if instruction.output {
                    instruction.output_value
                } else {
                    None
                }
            }
            Instruction::Jump(_instruction) => None,
        }
    }

    fn result_register(&self) -> Option<Register> {
        match self {
            Instruction::Division(instruction) => instruction.result_register,
            Instruction::Xor(instruction) => instruction.result_register,
            Instruction::Modulo(instruction) => instruction.result_register,
            Instruction::Jump(_instruction) => None,
        }
    }
}

fn read_input(path: &str) -> (Vec<usize>, Vec<Opcode>) {
    let input = fs::read_to_string(path).unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut registers = Vec::new();
    let mut program = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() == 0 {
            continue;
        }
        if parts[0] == "Register" {
            registers.push(parts[parts.len() - 1].parse().unwrap());
        } else if parts[0].starts_with("Program") {
            program = parts[parts.len() - 1]
                .split(',')
                .map(|s| Opcode::from(s.parse::<usize>().unwrap()))
                .collect::<Vec<_>>();
            break;
        }
    }
    (registers, program)
}

fn part_one(registers: &mut Vec<usize>, program: &[Opcode]) -> String {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();
    while instruction_pointer < program.len() {
        // Check if we have enough bytes for opcode + operand
        if instruction_pointer + 1 >= program.len() {
            break;
        }

        let opcode = &program[instruction_pointer];
        let raw_operand = program[instruction_pointer + 1].0;

        // Determine if this instruction uses combo or literal operand
        let operand = match opcode.0 {
            0 | 2 | 5 | 6 | 7 => {
                // These use combo operands
                Opcode::combo_operand(raw_operand, registers)
            }
            1 | 3 => {
                // These use literal operands
                raw_operand
            }
            4 => {
                // Operand is ignored
                raw_operand
            }
            _ => raw_operand,
        };

        let instruction_builder = opcode.instruction();
        let instruction = instruction_builder(operand);
        let result = instruction.execute(registers);

        // Check for output
        if let Some(output_value) = instruction.output() {
            output.push(output_value);
        }

        if result.is_some() {
            instruction_pointer = result.unwrap();
        } else {
            instruction_pointer += 2;
        }
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn test_a_value(a_value: usize, initial_registers: &[usize], program: &[Opcode]) -> Option<usize> {
    let mut registers = initial_registers.to_vec();
    registers[Register::A.index()] = a_value;
    let mut instruction_pointer = 0;
    let mut program_pointer = 0;
    let mut matched_all = true;
    
    while instruction_pointer < program.len() {
        if instruction_pointer + 1 >= program.len() {
            break;
        }

        let opcode = &program[instruction_pointer];
        let raw_operand = program[instruction_pointer + 1].0;

        // Determine if this instruction uses combo or literal operand
        let operand = match opcode.0 {
            0 | 2 | 5 | 6 | 7 => {
                // These use combo operands
                Opcode::combo_operand(raw_operand, &registers)
            }
            1 | 3 => {
                // These use literal operands
                raw_operand
            }
            4 => {
                // Operand is ignored
                raw_operand
            }
            _ => raw_operand,
        };

        let instruction_builder = opcode.instruction();
        let instruction = instruction_builder(operand);
        let result = instruction.execute(&mut registers);

        // Check for output
        if let Some(output_value) = instruction.output() {
            // If we've already matched all program values, check if there's extra output
            if program_pointer >= program.len() {
                matched_all = false;
                break;
            }
            
            // Check if output matches expected program value
            if program[program_pointer].0 != output_value {
                matched_all = false;
                break;
            }
            
            program_pointer += 1;
            
            // If we've matched all program values, we're done (even if program continues)
            if program_pointer == program.len() {
                break;
            }
        }

        if result.is_some() {
            instruction_pointer = result.unwrap();
        } else {
            instruction_pointer += 2;
        }
    }
    
    // Check if we matched all program values
    if matched_all && program_pointer == program.len() {
        Some(a_value)
    } else {
        None
    }
}

fn part_two(registers: &mut Vec<usize>, program: &[Opcode], stop_at: usize) -> usize {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Instant;
    
    let initial_registers = registers.clone();
    let start_time = Instant::now();
    let items_tested = AtomicUsize::new(0);
    let last_logged_percent = AtomicUsize::new(0);
    
    let result = (1..=stop_at)
        .into_par_iter()
        .find_first(|&a_value| {
            // Increment counter of items tested (approximate, but better than tracking max value)
            let tested = items_tested.fetch_add(1, Ordering::Relaxed) + 1;
            
            // Log progress based on number of items tested, not the value being tested
            // This gives a more accurate picture of actual progress
            let percent = (tested * 100) / stop_at.max(1);
            let last_percent = last_logged_percent.load(Ordering::Relaxed);
            
            // Log when we cross a new percentage threshold
            if percent > last_percent {
                // Try to update last_logged_percent (only one thread will succeed)
                if last_logged_percent.compare_exchange(
                    last_percent,
                    percent,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ).is_ok() {
                    let elapsed = start_time.elapsed();
                    println!(
                        "[{:.2}s] Progress: {}% (tested {} values, currently testing {})",
                        elapsed.as_secs_f64(),
                        percent,
                        tested,
                        a_value
                    );
                }
            }
            
            test_a_value(a_value, &initial_registers, program).is_some()
        });
    
    let elapsed = start_time.elapsed();
    let final_tested = items_tested.load(Ordering::Relaxed);
    println!("Total search time: {:.2}s (tested {} values)", elapsed.as_secs_f64(), final_tested);
    
    result.unwrap_or_else(|| panic!("No match found up to {}", stop_at))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (mut registers, program) = read_input("test.txt");
        assert_eq!(part_one(&mut registers, &program), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two_example() {
        // Test with the example from the problem: program 0,3,5,4,3,0 with A=117440
        let mut registers = vec![2024, 0, 0];
        let program = vec![Opcode(0), Opcode(3), Opcode(5), Opcode(4), Opcode(3), Opcode(0)];
        let result = part_two(&mut registers, &program, 200000);
        assert_eq!(result, 117440, "Example program should find 117440");
    }

    #[test]
    fn test_part_two() {
        let (mut registers, program) = read_input("test2.txt");
        // Find the actual correct value for test.txt program
        let result = part_two(&mut registers, &program, 200000);
        println!("Found A value for test.txt: {}", result);
        // Verify it works
        registers[Register::A.index()] = result;
        let output = part_one(&mut registers, &program);
        let expected: Vec<String> = program.iter().map(|o| o.0.to_string()).collect();
        let expected_str = expected.join(",");
        assert_eq!(output, expected_str, "Output should match program");
    }
}

fn main() {
    let (mut registers, program) = read_input("input.txt");
    println!("Part one: {}", part_one(&mut registers, &program));
    println!("Part two: {}", part_two(&mut registers, &program, 1_000_000_000_000));
}
