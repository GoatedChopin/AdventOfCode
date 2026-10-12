use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Clone, Copy)]
enum Operation {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

impl Operation {
    fn execute(&self, left: usize, right: Option<usize>) -> usize {
        use Operation::*;
        match *self {
            And => {
                let right = right.expect("Right must not be none in And");
                left & right
            }
            Or => {
                let right = right.expect("Right must not be none in Or");
                left | right
            }
            Lshift => {
                let right = right.expect("Right must not be none in Lshift");
                left << right
            }
            Rshift => {
                let right = right.expect("Right must not be none in Rshift");
                left >> right
            }
            Not => !left,
        }
    }
}

impl Operation {
    fn parse(input: &str) -> Option<Self> {
        use Operation::*;
        match input {
            "AND" => Some(And),
            "OR" => Some(Or),
            "NOT" => Some(Not),
            "LSHIFT" => Some(Lshift),
            "RSHIFT" => Some(Rshift),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Input {
    Literal(usize),
    Wire(String),
}

impl Input {
    fn parse(input: &str) -> Self {
        let usize_result = input.parse::<usize>();
        match usize_result {
            Ok(u) => Input::Literal(u),
            Err(_) => Input::Wire(input.to_string()),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Wire {
    raw: String,
    left: Input,
    right: Option<Input>,
    operation: Option<Operation>,
    output: String,
}

impl Wire {
    fn parse(line: &str) -> Result<Self, &str> {
        let Some((left, right)) = line.split_once("->") else {
            return Err("Bad wire, no arrow");
        };
        let output = right.trim().to_string();
        let mut left_option = None;
        let mut right_option = None;
        let mut operation = None;
        for token in left.split_whitespace() {
            let input_result = Input::parse(token);
            let operation_result = Operation::parse(token);
            match (input_result, operation_result) {
                (_, Some(o)) => {
                    operation = Some(o);
                }
                (input, _) => {
                    if left_option.is_none() {
                        left_option = Some(input);
                        continue;
                    }
                    if right_option.is_none() {
                        right_option = Some(input);
                        continue;
                    }
                }
            }
        }
        let Some(left) = left_option else {
            return Err("Bad wire, no left input");
        };
        let right = right_option;
        Ok(Self {
            raw: line.to_string(),
            left,
            right,
            operation,
            output,
        })
    }

    fn ready(&self, known_values: &HashMap<String, usize>) -> bool {
        if self.right.is_none() {
            return match &self.left {
                Input::Literal(_) => true,
                Input::Wire(w) => known_values.contains_key(w),
            };
        }
        match (&self.left, &self.right) {
            (l, Some(r)) => {
                let left = match l {
                    Input::Literal(_) => true,
                    Input::Wire(w) => known_values.contains_key(w),
                };
                let right = match r {
                    Input::Literal(_) => true,
                    Input::Wire(w) => known_values.contains_key(w),
                };
                left && right
            }
            (_, _) => false,
        }
    }

    fn execute(&self, known_values: &HashMap<String, usize>) -> usize {
        let left = match &self.left {
            Input::Literal(l) => l,
            Input::Wire(w) => known_values
                .get(w)
                .expect("Value must be known to execute a wire"),
        };
        if self.operation.is_none() {
            return *left;
        }
        let right = match self.right.clone() {
            None => None,
            Some(Input::Literal(l)) => Some(l),
            Some(Input::Wire(w)) => match known_values.get(&w) {
                Some(u) => Some(*u),
                None => None,
            },
        };
        self.operation
            .expect("Operation must not be none at this branch")
            .execute(*left, right)
    }
}

impl Hash for Wire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

#[derive(Clone)]
struct Circuit {
    wires: HashMap<String, Wire>,
    wires_needed: HashMap<Wire, HashSet<String>>,
    known_values: HashMap<String, usize>,
}

impl Circuit {
    fn run(&mut self, unlocked_wires: HashSet<Wire>) {
        for wire in unlocked_wires {
            let output_value = wire.execute(&self.known_values);
            self.known_values.insert(wire.output, output_value);
        }
    }

    fn set_wire(&mut self, wire: String, val: usize) {
        self.known_values.insert(wire.clone(), val);
        let bad_wires: Vec<String> = self
            .wires
            .iter()
            .filter(|(_, w)| w.output == wire)
            .map(|(raw, _)| raw.clone())
            .collect();

        for raw in bad_wires {
            if let Some(old_wire) = self.wires.remove(&raw) {
                self.wires_needed.remove(&old_wire);
            }
        }
        for (_, wires_needed) in self.wires_needed.iter_mut() {
            if !wires_needed.contains(&wire) {
                continue;
            }
            wires_needed.remove(&wire);
        }

        let new_wire_raw = format!("{} -> {}", val, wire);
        let new_wire = Wire::parse(&new_wire_raw).expect("Literal inputs must be valid");
        println!(
            "Wire {} will now evaluate to {}",
            new_wire_raw,
            new_wire.execute(&HashMap::new())
        );
        self.wires_needed.insert(new_wire.clone(), HashSet::new());
        self.wires.insert(new_wire_raw, new_wire);
    }
}

fn read_inputs(inputs: &str) -> Circuit {
    let bytes = fs::read_to_string(inputs).expect("No input file");
    let lines = bytes.lines();
    let mut wires = HashMap::new();
    let mut wires_needed = HashMap::new();
    let mut known_values = HashMap::new();
    for line in lines {
        let Ok(wire) = Wire::parse(line) else {
            println!("Bad wire: {}", line);
            continue;
        };
        let mut wn = HashSet::new();
        match wire.left.clone() {
            Input::Wire(w) => {
                wn.insert(w);
            }
            Input::Literal(l) => {
                if wire.right.is_none() {
                    known_values.insert(wire.output.clone(), l);
                }
            }
        }
        match wire.right.clone() {
            Some(Input::Wire(w)) => {
                wn.insert(w);
            }
            _ => {}
        }
        wires.insert(line.to_string(), wire.clone());
        wires_needed.insert(wire, wn);
    }
    Circuit {
        wires,
        wires_needed,
        known_values,
    }
}

fn part_one(circuit: Circuit, target_wire: String) -> usize {
    let mut circuit = circuit;
    while !circuit.known_values.contains_key(&target_wire) {
        let mut unlocked_wires = HashSet::new();
        for (w, _) in circuit.known_values.iter() {
            for (wire, wires_needed) in circuit.wires_needed.iter_mut() {
                if wires_needed.contains(w) {
                    wires_needed.remove(w);
                }
                if wire.ready(&circuit.known_values) {
                    unlocked_wires.insert(wire.clone());
                }
            }
        }
        circuit.run(unlocked_wires);
    }
    circuit.known_values[&target_wire]
}

fn main() {
    let target_wire = "a".to_string();
    let circuit = read_inputs("input.txt");
    let p1 = part_one(circuit.clone(), target_wire.clone());
    println!("Part one: {}", p1);
    let mut circuit = read_inputs("input.txt");
    circuit.set_wire("b".to_string(), p1);
    let p2 = part_one(circuit, target_wire);
    println!("Part two: {}", p2);
}
