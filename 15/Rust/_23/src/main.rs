use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Jump {
    size: usize,
    forward: bool,
}

impl From<&str> for Jump {
    fn from(s: &str) -> Jump {
        let forward = s.contains("+");
        Jump {
            size: s
                .replace("+", "")
                .replace("-", "")
                .parse()
                .expect("Bad Jump value"),
            forward,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl TryFrom<&str> for Register {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Register, Self::Error> {
        if s == "a" {
            return Ok(Register::A);
        }
        if s == "b" {
            return Ok(Register::B);
        }
        Err("Bad register value")
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Jump),
    JumpIfEven((Register, Jump)),
    JumpIfOne((Register, Jump)),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let scrubbed = s.replace(",", "");
        let parts: Vec<&str> = scrubbed.split_whitespace().collect();
        if parts.len() < 2 {
            panic!("Bad Instruction line");
        }
        match parts[0] {
            "hlf" => {
                let register = Register::try_from(parts[1]).expect("Bad hlf register value");
                Instruction::Half(register)
            }
            "tpl" => {
                let register = Register::try_from(parts[1]).expect("Bad tpl register value");
                Instruction::Triple(register)
            }
            "inc" => {
                let register = Register::try_from(parts[1]).expect("Bad inc register value");
                Instruction::Increment(register)
            }
            "jmp" => {
                let jump = Jump::from(parts[1]);
                Instruction::Jump(jump)
            }
            "jie" => {
                let register = Register::try_from(parts[1]).expect("Bad inc register value");
                let jump = Jump::from(parts[2]);
                Instruction::JumpIfEven((register, jump))
            }
            "jio" => {
                let register = Register::try_from(parts[1]).expect("Bad inc register value");
                let jump = Jump::from(parts[2]);
                Instruction::JumpIfOne((register, jump))
            }
            _ => panic!("Bad instruction value"),
        }
    }
}

#[derive(Clone, Debug)]
struct VM {
    pointer: usize,
    registers: HashMap<Register, usize>,
    instructions: Vec<Instruction>,
}

impl VM {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            pointer: 0,
            registers: HashMap::from([(Register::A, 0), (Register::B, 0)]),
            instructions,
        }
    }

    fn step(&mut self) -> Result<usize, &str> {
        let Some(instruction) = self.instructions.get(self.pointer) else {
            return Err("Instruction pointer out of bounds");
        };
        match instruction {
            Instruction::Half(r) => {
                let Some(register) = self.registers.get_mut(r) else {
                    return Err("Bad register");
                };
                *register = *register / 2;
                self.pointer += 1;
            }
            Instruction::Triple(r) => {
                let Some(register) = self.registers.get_mut(r) else {
                    return Err("Bad register");
                };
                *register = *register * 3;
                self.pointer += 1;
            }
            Instruction::Increment(r) => {
                let Some(register) = self.registers.get_mut(r) else {
                    return Err("Bad register");
                };
                *register = *register + 1;
                self.pointer += 1;
            }
            Instruction::Jump(j) => match j.forward {
                true => {
                    self.pointer += j.size;
                }
                false => {
                    if j.size > self.pointer {
                        return Err("Pointer out of bounds, bad jmp");
                    }
                    self.pointer -= j.size;
                }
            },
            Instruction::JumpIfEven((r, j)) => {
                let Some(register) = self.registers.get(r) else {
                    return Err("Bad register");
                };
                if *register % 2 == 0 {
                    match j.forward {
                        true => {
                            self.pointer += j.size;
                        }
                        false => {
                            if j.size > self.pointer {
                                return Err("Pointer out of bounds, bad jmp");
                            }
                            self.pointer -= j.size;
                        }
                    }
                } else {
                    self.pointer += 1;
                }
            }
            Instruction::JumpIfOne((r, j)) => {
                let Some(register) = self.registers.get(r) else {
                    return Err("Bad register");
                };
                if *register == 1 {
                    match j.forward {
                        true => {
                            self.pointer += j.size;
                        }
                        false => {
                            if j.size > self.pointer {
                                return Err("Pointer out of bounds, bad jmp");
                            }
                            self.pointer -= j.size;
                        }
                    }
                } else {
                    self.pointer += 1;
                }
            }
        };

        Ok(self.pointer)
    }

    fn execute(&mut self) {
        loop {
            match self.step() {
                Ok(_) => {}
                Err(_e) => {
                    break;
                }
            }
        }
    }
}

fn read_input(path: &str) -> Vec<Instruction> {
    let s = fs::read_to_string(path).expect("Input must exist");
    s.lines().map(|line| Instruction::from(line)).collect()
}

fn part_one(vm: VM) -> usize {
    let mut vm = vm.clone();
    vm.execute();
    *vm.registers
        .get(&Register::B)
        .expect("Register B must exist")
}

fn main() {
    let instructions = read_input("input.txt");
    let mut vm = VM::new(instructions);
    println!("Part one: {}", part_one(vm.clone()));
    let Some(a) = vm.registers.get_mut(&Register::A) else {
        panic!();
    };
    *a = 1;
    println!("Part two: {}", part_one(vm.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execution() {
        let example = "inc a
        jio a, +2
        tpl a
        inc a";
        let instructions = example
            .lines()
            .map(|line| Instruction::from(line))
            .collect();
        let mut vm = VM::new(instructions);
        vm.step();
        assert_eq!(
            *vm.registers
                .get(&Register::A)
                .expect("Register A must exist"),
            1
        );
        vm.step();
        assert_eq!(vm.pointer, 3);
        vm.step();
        assert_eq!(
            *vm.registers
                .get(&Register::A)
                .expect("Register A must exist"),
            2
        );
        assert!(vm.step().is_err());
    }
}
