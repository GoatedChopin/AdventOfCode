use std::collections::HashMap;
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
    history: Vec<T>,
    is_literal: bool,
}

impl TryFrom<&str> for Register<isize> {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Register<isize>, Self::Error> {
        Err("Bad time")
    }
}

#[derive(Clone)]
enum Comparison {
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone)]
struct Condition<T> {
    left: Register<T>,
    comparison: Comparison,
    right: Register<T>,
}

#[derive(Clone)]
enum Instruction<T> {
    Send(Register<T>),
    Set(Register<T>, Register<T>),
    Add(Register<T>, Register<T>),
    Multiply(Register<T>, Register<T>),
    Modulate(Register<T>, Register<T>),
    Receive(Register<T>),
    JumpIf(Register<T>, Register<T>, Condition<T>),
}

impl Instruction<T> {
    fn registers(&self) -> Vec<Register> {
        use Instruction::*;
        let mut out = Vec::new();
        match self {
            Send(r) => {
                if !r.is_literal {
                    out.push(r.clone());
                }
            }
            Set(r1, r2) => {}
            Add(r1, r2) => {}
            Multiply(r1, r2) => {}
            Modulate(r1, r2) => {}
            Receive(r) => {}
            JumpIf(r1, r2, condition) => {}
        };
        out
    }
}

fn read_input(path: &str) -> Emulator {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let instructions = s.lines().map(|line| line.into()).collect();
    let registers = HashMap::from(
        instructions
            .iter()
            .filter(|instruction| instruction.registers()),
    );
    Emulator {
        registers,
        instructions,
    }
}

fn main() {
    println!("Hello, world!");
}
