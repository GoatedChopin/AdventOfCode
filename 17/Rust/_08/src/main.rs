use std::collections::HashMap;
use std::fs;

struct Instruction {
    register: String,
    operation: String,
    value: i32,
    condition_register: String,
    condition_operation: String,
    condition_value: i32,
}

fn read_input(path: &str) -> Vec<Instruction> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let register = parts[0].to_string();
            let operation = parts[1].to_string();
            let value = parts[2].parse::<i32>().unwrap();
            let condition_register = parts[4].to_string();
            let condition_operation = parts[5].to_string();
            let condition_value = parts[6].parse::<i32>().unwrap();
            Instruction {
                register,
                operation,
                value,
                condition_register,
                condition_operation,
                condition_value,
            }
        })
        .collect::<Vec<Instruction>>()
}

fn follow_instruction(instruction: &Instruction, registers: &mut HashMap<String, i32>) {
    let condition_value = registers.get(&instruction.condition_register).unwrap_or(&0);
    let valent = if instruction.operation == "inc" { 1 } else { -1 };
    match instruction.condition_operation.as_str() {
        ">" => {
            if *condition_value > instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        "<" => {
            if *condition_value < instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        ">=" => {
            if *condition_value >= instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        "<=" => {
            if *condition_value <= instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        "==" => {
            if *condition_value == instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        "!=" => {
            if *condition_value != instruction.condition_value {
                registers.insert(
                    instruction.register.clone(),
                    registers.get(&instruction.register).unwrap_or(&0) + (instruction.value * valent),
                );
            }
        }
        _ => {
            panic!(
                "Invalid condition operation: {}",
                instruction.condition_operation
            );
        }
    }
}

fn part_one(instructions: &Vec<Instruction>) -> i32 {
    let mut registers = HashMap::new();
    for instruction in instructions {
        follow_instruction(instruction, &mut registers);
    }
    *registers.values().max().unwrap()
}

fn part_two(instructions: &Vec<Instruction>) -> i32 {
  let mut registers = HashMap::new();
  let mut max = 0;
  for instruction in instructions {
      follow_instruction(instruction, &mut registers);
      let current = *registers.values().max().unwrap();
      if current > max {
        max = current;
      }
  }
  max
}

fn main() {
    test();
    let instructions = read_input("input.txt");
    println!("Part one: {}", part_one(&instructions));
    println!("Part two: {}", part_two(&instructions));
}

fn test() {
    let instructions = read_input("test.txt");
    assert_eq!(part_one(&instructions), 1);
}