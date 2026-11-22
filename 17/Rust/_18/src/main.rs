use std::cmp::{Eq, PartialEq, PartialOrd};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::ops::{Add, Mul, Rem};

enum Execution<T> {
    Receive(T),
}

#[derive(Clone)]
struct Emulator<T> {
    registers: HashMap<char, Register<T>>,
    instructions: Vec<Instruction<T>>,
    stack_pointer: usize,
}

impl<
    T: TryInto<usize>
        + PartialEq
        + Eq
        + PartialOrd
        + Add<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + Copy,
> Emulator<T>
{
    fn get_value(&self, register: char) -> Option<T> {
        match self.registers.get(&register) {
            Some(r) => Some(r.value),
            None => None,
        }
    }

    fn execute_next_instruction(&mut self) -> Option<Execution<T>> {
        use Instruction::*;
        let instruction = self.instructions[self.stack_pointer];
        match instruction {
            Send(r) => {
                let Some(register) = self.registers.get_mut(&r) else {
                    return None;
                };
                register.history.push_front(register.value);
                if register.history.len() > 1 {
                    register.history.pop_back();
                }
                None
            }
            Set(left, right) => {
                let Some(right_value) = (match right {
                    Value::Register(right_register) => self.get_value(right_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };
                let Some(register) = self.registers.get_mut(&left) else {
                    return None;
                };
                register.value = right_value;
                None
            }
            Add(left, right) => {
                let Some(right_value) = (match right {
                    Value::Register(right_register) => self.get_value(right_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };
                let Some(register) = self.registers.get_mut(&left) else {
                    return None;
                };
                register.value = register.value + right_value;
                None
            }
            Multiply(left, right) => {
                let Some(right_value) = (match right {
                    Value::Register(right_register) => self.get_value(right_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };
                let Some(register) = self.registers.get_mut(&left) else {
                    return None;
                };
                register.value = register.value * right_value;
                None
            }
            Modulate(left, right) => {
                let Some(right_value) = (match right {
                    Value::Register(right_register) => self.get_value(right_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };
                let Some(register) = self.registers.get_mut(&left) else {
                    return None;
                };
                register.value = register.value % right_value;
                None
            }
            Receive(r) => {
                if let Some(value) = self.get_value(r) {
                    return Some(Execution::Receive(value));
                };
                None
            }
            JumpIf(left, right, condition) => {
                let Some(left_value) = (match left {
                    Value::Register(left_register) => self.get_value(left_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };
                let Some(jump_by) = (match right {
                    Value::Register(right_register) => self.get_value(right_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };

                let Some(left_compare) = (match condition.left {
                    Value::Register(left_register) => self.get_value(left_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };

                let Some(right_compare) = (match condition.left {
                    Value::Register(left_register) => self.get_value(left_register),
                    Value::Literal(literal) => Some(literal),
                }) else {
                    return None;
                };

                let should_jump = match condition.comparator {
                    Comparator::EqualTo => left_compare == right_compare,
                    Comparator::GreaterThan => left_compare > right_compare,
                    Comparator::LessThan => left_compare < right_compare,
                };
                if !should_jump {
                    return None;
                }
                let Ok(ujump) = jump_by.try_into() else {
                    return None;
                };
                self.stack_pointer = (self.stack_pointer + ujump) % self.instructions.len();
                None
            }
        }
    }
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
        let Some(name) = s.chars().next() else {
            return Err("Cannot build Value from empty &str");
        };
        Ok(Register::new(name, 0))
    }
}

impl<T> Register<T> {
    fn new(name: char, value: T) -> Self {
        Self {
            name,
            value,
            history: VecDeque::new(),
        }
    }
}

#[derive(Clone, Copy)]
enum Comparator {
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Copy)]
struct Condition<T> {
    left: Value<T>,
    comparator: Comparator,
    right: Value<T>,
}

#[derive(Clone, Copy)]
enum Value<T> {
    Register(char),
    Literal(T),
}

impl TryFrom<&str> for Value<isize> {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Value<isize>, Self::Error> {
        match s.parse() {
            Err(_) => {
                let Some(name) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                Ok(Value::Register(name))
            }
            Ok(i) => Ok(Value::Literal(i)),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction<T> {
    Send(char),
    Set(char, Value<T>),
    Add(char, Value<T>),
    Multiply(char, Value<T>),
    Modulate(char, Value<T>),
    Receive(char),
    JumpIf(Value<T>, Value<T>, Condition<T>),
}

impl TryFrom<&str> for Instruction<isize> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Instruction<isize>, Self::Error> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts[0] {
            "snd" => {
                let Some(name) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                Ok(Instruction::Send(name))
            }
            "set" => {
                let Some(left) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Set(left, right))
            }
            "add" => {
                let Some(left) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Add(left, right))
            }
            "mul" => {
                let Some(left) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Multiply(left, right))
            }
            "mod" => {
                let Some(left) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::Modulate(left, right))
            }
            "rcv" => {
                let Some(register) = s.chars().next() else {
                    return Err("Cannot build Value from empty &str");
                };
                Ok(Instruction::Receive(register))
            }
            "jgz" => {
                let left = match Value::try_from(parts[1]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                let right = match Value::try_from(parts[2]) {
                    Ok(value) => value,
                    Err(_) => return Err("Bad right side set register value"),
                };
                Ok(Instruction::JumpIf(
                    left.clone(),
                    right,
                    Condition {
                        left,
                        comparator: Comparator::GreaterThan,
                        right: Value::Literal(0),
                    },
                ))
            }
            _ => Err("Bad instruction"),
        }
    }
}

impl<T: Clone> Instruction<T> {
    fn registers(&self) -> Vec<char> {
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
            JumpIf(r1, r2, _) => {
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
    let registers = HashMap::from_iter(
        instructions
            .iter()
            .filter_map(|instruction| {
                let instruction_registers = instruction.registers();
                match instruction_registers.len() == 0 {
                    true => None,
                    false => Some(
                        instruction_registers
                            .into_iter()
                            .map(|name| (name, Register::new(name, 0))),
                    ),
                }
            })
            .flatten()
            .collect::<Vec<(char, Register<isize>)>>(),
    );
    Emulator {
        registers,
        instructions,
        stack_pointer: 0,
    }
}

fn part_one(emulator: Emulator<isize>) -> isize {
    0
}

fn main() {
    let emulator = read_input("input.txt");
    println!("Part one: {}", part_one(emulator));
}
