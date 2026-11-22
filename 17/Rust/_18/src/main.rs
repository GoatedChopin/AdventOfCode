use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Clone)]
struct Emulator<T> {
    registers: HashMap<char, Register<T>>,
    instructions: Vec<Instruction<T>>,
}

#[derive(Clone)]
struct Register<T> {
    name: char,
    value: T,
    history: VecDeque<T>,
}

impl TryFrom<&str> for Register<isize> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Register<isize>, Self::Error> {
        Err("Bad time")
    }
}

impl<T> Register<T> {
    fn new(name: char, value: T) -> Self<T> {
        Self {
            name,
            value,
            history: VecDeque::new(),
        }
    }
}

#[derive(Clone)]
enum Comparator {
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone)]
struct Condition<T> {
    left: Value<T>,
    comparison: Comparator,
    right: Value<T>,
}

#[derive(Clone)]
enum Value<T> {
    Register(Register<T>),
    Literal(T),
}

impl TryFrom<&str> for Value<isize> {
    type Error = &'static str;
    fn try_from(s: &str) -> Value<isize> {
        match s.parse() {
            Err(e) => {
                let name: char = s[0..1].into();
                Value::Register(Register::new(name, 0))
            }
            Ok(i) => Value::Literal(i),
        }
}

#[derive(Clone)]
enum Instruction<T> {
    Send(Register<T>),
    Set(Register<T>, Value<T>),
    Add(Register<T>, Value<T>),
    Multiply(Register<T>, Value<T>),
    Modulate(Register<T>, Value<T>),
    Receive(Register<T>),
    JumpIf(Value<T>, Value<T>, Condition<T>),
}

impl TryFrom<&str> for Instruction<isize> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Instruction<isize>, Self::Error> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts[0] {
            "snd" => {
                let register = Register::new(parts[1][0..1].into(), 0);
                Ok(Instruction::Send(register))
            }
            "set" => {
                let left = Register::new(parts[1][0..1].into(), 0);
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Set(left, right))
            }
            "add" => {
                let left = Register::new(parts[1][0..1].into(), 0);
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Add(left, right))
            }
            "mul" => {
                let left = Register::new(parts[1][0..1].into(), 0);
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Multiply(left, right))
            }
            "mod" => {
                let left = Register::new(parts[1][0..1].into(), 0);
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Modulate(left, right))
            }
            "rcv" => {
                let register = Register::new(parts[1][0..1].into(), 0);
                Ok(Instruction::Receive(register))
            }
            "jgz" => {
                let left = match Value::try_from(parts[1]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(e) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::JumpIf(left.clone(), right, Condition { left, comparator: Comparator::GreaterThan, right: Value::Literal(0) }))
            }
            _ => Err("Bad instruction"),
        }
    }
}

impl<T: Clone> Instruction<T> {
    fn registers(&self) -> Vec<Register<T>> {
        use Instruction::*;
        let mut out = Vec::new();
        match self {
            Send(r) => {
                out.push(r.clone());
            }
            Set(r1, r2) => {
                out.push(r1.clone());
                match r2 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
            }
            Add(r1, r2) => {
                out.push(r1.clone());
                match r2 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
            }
            Multiply(r1, r2) => {
                out.push(r1.clone());
                match r2 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
            }
            Modulate(r1, r2) => {
                out.push(r1.clone());
                match r2 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
            }
            Receive(r) => {
                out.push(r.clone());
            }
            JumpIf(r1, r2, condition) => {
                match r1 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
                match r2 {
                    Value::Register(r) => {
                        out.push(r.clone());
                    }
                    _ => {}
                };
            }
        };
        out
    }
}

fn read_input(path: &str) -> Emulator<isize> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let instructions: Vec<Instruction<isize>> = s
        .trim()
        .lines()
        .map(|line| line.try_into().expect("Bad instruction input"))
        .collect();
    let registers = HashMap::from(
        instructions
            .iter()
            .filter_map(|instruction| {
                let instruction_registers = instruction.registers();
                match instruction_registers.len() == 0 {
                    true => None,
                    false => Some(
                        instruction_registers
                            .into_iter()
                            .map(|register| (register.name, register)),
                    ),
                }
            })
            .flatten()
            .collect(),
    );
    Emulator {
        registers,
        instructions,
    }
}

fn main() {
    let emulator = read_input("input.txt");
    println!("Part one: {}", part_one(emulator));
}
