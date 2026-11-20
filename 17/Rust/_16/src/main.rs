use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Clone, Copy)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Clone)]
struct DanceCircle {
    positions: VecDeque<char>,
    instructions: Vec<Instruction>,
}

impl DanceCircle {
    fn execute(&mut self, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Spin(u) => {
                for _ in 0..*u {
                    let Some(back) = self.positions.pop_back() else {
                        return;
                    };
                    self.positions.push_front(back);
                }
            }
            Exchange(a, b) => {
                self.positions.swap(*a, *b);
            }
            Partner(a, b) => {
                let Some(ai) = self
                    .positions
                    .iter()
                    .enumerate()
                    .find_map(|(i, p)| if *p == *a { Some(i) } else { None })
                else {
                    return;
                };
                let Some(bi) = self
                    .positions
                    .iter()
                    .enumerate()
                    .find_map(|(i, p)| if *p == *b { Some(i) } else { None })
                else {
                    return;
                };
                self.positions.swap(ai, bi);
            }
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Instruction, Self::Error> {
        if s.len() == 0 {
            return Err("Instruction cannot be empty string");
        }
        let Some(leading_char) = s.chars().nth(0) else {
            return Err("Instruction cannot be an empty string");
        };
        match leading_char {
            'x' => {
                let Some((a, b)) = s[1..].trim().split_once("/") else {
                    return Err("No / in exchange instruction");
                };
                let Ok(ai) = a.parse() else {
                    return Err("Bad A integer in exchange instruction");
                };
                let Ok(bi) = b.parse() else {
                    return Err("Bad B integer in exchange instruction");
                };
                return Ok(Instruction::Exchange(ai, bi));
            }
            's' => {
                let Ok(u) = s[1..].trim().parse() else {
                    return Err("Bad integer in spin instruction");
                };
                return Ok(Instruction::Spin(u));
            }
            'p' => {
                let Some((a, b)) = s[1..].trim().split_once("/") else {
                    return Err("No / in exchange instruction");
                };
                let Ok(ai) = a.parse() else {
                    return Err("Bad A char in partner instruction");
                };
                let Ok(bi) = b.parse() else {
                    return Err("Bad B char in partner instruction");
                };
                return Ok(Instruction::Partner(ai, bi));
            }
            _ => return Err("Bad instruction leading char"),
        }
    }
}

fn read_input(path: &str) -> DanceCircle {
    let s = fs::read_to_string(path).expect("Input file must exist");
    DanceCircle {
        positions: VecDeque::from([
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ]),
        instructions: s
            .trim()
            .split(",")
            .map(|part| part.try_into().expect("Bad instruction"))
            .collect(),
    }
}

fn part_one(dance_circle: DanceCircle) -> String {
    let mut dance_circle = dance_circle;
    for instruction in dance_circle.instructions.clone() {
        dance_circle.execute(&instruction);
    }
    dance_circle.positions.into_iter().collect()
}

fn part_two(dance_circle: DanceCircle) -> String {
    let mut visited = HashMap::new();
    let mut dance_circle = dance_circle;
    let instructions = dance_circle.instructions.clone();
    let mut instruction_index = 0;
    let mut iterations = 0;
    loop {
        let current: String = dance_circle.positions.iter().map(|c| *c).collect();
        if visited.contains_key(&(current.clone(), instruction_index)) {
            break;
        }
        visited.insert((current, instruction_index), iterations);
        dance_circle.execute(&instructions[instruction_index]);
        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }
        iterations += 1;
    }

    for _ in 0..(1_000_000_000 % iterations) {
        dance_circle.execute(&instructions[instruction_index]);
        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }
    }
    dance_circle.positions.into_iter().collect()
}

fn main() {
    let dance_circle = read_input("input.txt");
    println!("Part one: {}", part_one(dance_circle.clone()));
    println!("Part two: {}", part_two(dance_circle));
}
