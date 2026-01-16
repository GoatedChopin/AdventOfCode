use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::BitAndAssign,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Register {
    name: [char; 3],
}

impl Register {
    fn from_str(s: &str) -> Register {
        Register {
            name: s.chars().collect::<Vec<char>>().try_into().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl Operation {
    fn execute(&self, left: bool, right: bool) -> bool {
        match self {
            Operation::AND => left && right,
            Operation::OR => left || right,
            Operation::XOR => left ^ right,
        }
    }

    fn from_str(s: &str) -> Option<Operation> {
        match s {
            "AND" => Some(Operation::AND),
            "OR" => Some(Operation::OR),
            "XOR" => Some(Operation::XOR),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Wire {
    left: Register,
    right: Register,
    operation: Operation,
    output: Register,
}

#[derive(Debug, Clone)]
struct Circuit {
    wires: Vec<Wire>,
    registers: HashMap<[char; 3], Option<bool>>,
}

enum InputState {
    Registers,
    Wires,
}

fn read_input(file_path: &str) -> Circuit {
    let content = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines: Vec<&str> = content.split("\n").collect();
    let mut input_state = InputState::Registers;
    let mut circuit = Circuit {
        wires: Vec::new(),
        registers: HashMap::new(),
    };
    for line in lines.iter() {
        if line.is_empty() {
            input_state = InputState::Wires;
            continue;
        }
        match input_state {
            InputState::Registers => {
                let parts: Vec<&str> = line.split(": ").collect();
                let register: [char; 3] =
                    parts[0].chars().collect::<Vec<char>>().try_into().unwrap();
                let value: bool = if parts[1] == "1" { true } else { false };
                circuit.registers.insert(register, Some(value));
            }
            InputState::Wires => {
                let parts: Vec<&str> = line.split(" ").collect();
                let left: Register = Register::from_str(parts[0]);
                let right: Register = Register::from_str(parts[2]);
                let operation = Operation::from_str(parts[1]).unwrap();
                let output: Register = Register::from_str(parts[4]);

                // If left, right, or output are not in the Registers, put them in there with a None value.
                if !circuit.registers.contains_key(&left.name) {
                    circuit.registers.insert(left.name, None);
                }
                if !circuit.registers.contains_key(&right.name) {
                    circuit.registers.insert(right.name, None);
                }
                if !circuit.registers.contains_key(&output.name) {
                    circuit.registers.insert(output.name, None);
                }

                circuit.wires.push(Wire {
                    left,
                    right,
                    operation,
                    output,
                });
            }
        }
    }
    circuit
}

fn evaluate_circuit(circuit: &Circuit) -> usize {
    let mut z_registers = circuit
        .registers
        .iter()
        .filter(|(register, _)| register[0] == 'z')
        .collect::<Vec<_>>();
    z_registers.sort_by_key(|(register, _)| *register);
    from_binary(
        z_registers
            .iter()
            .map(|(_, value)| value.unwrap_or(false))
            .collect::<Vec<bool>>(),
    )
}

fn part_one(circuit: &mut Circuit) -> usize {
    // Iteratively gather all of the wires that are newly ready to be evaluated.
    // Execute those, store the new values in the output registers.
    // Repeat until all wires are evaluated.
    // Return evaluate_circuit
    let mut visited = HashSet::new();
    let mut ready_wires = circuit
        .wires
        .iter()
        .filter(|wire| !visited.contains(&(wire.left.name, wire.operation, wire.right.name)))
        .filter(|wire| {
            circuit.registers.get(&wire.left.name).is_some()
                && circuit.registers.get(&wire.left.name).unwrap().is_some()
                && circuit.registers.get(&wire.right.name).is_some()
                && circuit.registers.get(&wire.right.name).unwrap().is_some()
        })
        .collect::<Vec<_>>();

    while !ready_wires.is_empty() {
        ready_wires.iter().for_each(|wire| {
            let left_register = circuit.registers.get(&wire.left.name);
            if left_register.is_none() {
                println!("Left register is none for wire: {:?}", wire);
            }
            let right_register = circuit.registers.get(&wire.right.name);
            if right_register.is_none() {
                println!("Right register is none for wire: {:?}", wire);
            }
            let left_value = left_register.unwrap();
            let right_value = right_register.unwrap();
            if left_value.is_none() || right_value.is_none() {
                // println!("Left or right register value is none for wire: {:?}", wire);
                return;
            }
            let left = left_value.unwrap();
            let right = right_value.unwrap();
            let result = wire.operation.execute(left, right);
            circuit.registers.insert(wire.output.name, Some(result));
            visited.insert((wire.left.name, wire.operation, wire.right.name));
        });
        ready_wires = circuit
            .wires
            .iter()
            .filter(|wire| !visited.contains(&(wire.left.name, wire.operation, wire.right.name)))
            .filter(|wire| {
                circuit.registers.get(&wire.left.name).is_some()
                    && circuit.registers.get(&wire.right.name).is_some()
            })
            .collect::<Vec<_>>();
    }

    evaluate_circuit(circuit)
}

fn from_binary(bits: Vec<bool>) -> usize {
    let mut n = 0;
    for (i, bit) in bits.iter().enumerate() {
        if *bit {
            n += 2_usize.pow(i as u32);
        }
    }
    n
}

fn to_binary(n: usize) -> Vec<bool> {
    let mut n = n;
    if n == 0 {
        return vec![false];
    }

    let mut bits = Vec::with_capacity(64); // or 32 on 32-bit targets

    while n > 0 {
        bits.push(n & 1 == 1);
        n >>= 1;
    }

    bits // LSB at index 0
}

fn get_register_number(circuit: &Circuit, prefix: char) -> Vec<bool> {
    let mut z_registers = circuit
        .registers
        .iter()
        .filter(|(register, _)| register[0] == prefix)
        .collect::<Vec<_>>();
    z_registers.sort_by_key(|(register, _)| *register);
    z_registers
        .iter()
        .map(|(_, value)| value.unwrap_or(false))
        .collect::<Vec<bool>>()
}

fn register_name(prefix: char, index: usize) -> [char; 3] {
    let char_1 = if index < 10 {
        '0'
    } else {
        (index / 10).to_string().chars().next().unwrap()
    };
    let char_2 = if index < 10 {
        index.to_string().chars().next().unwrap()
    } else {
        (index % 10).to_string().chars().next().unwrap()
    };
    [prefix, char_1, char_2]
}

fn part_two(circuit: &mut Circuit) -> String {
    // Identify the x and y registers' input numbers
    let x_num = get_register_number(circuit, 'x');
    let y_num = get_register_number(circuit, 'y');
    let expected_z_num = to_binary(from_binary(x_num) + from_binary(y_num));

    let mut temp_circuit = circuit.clone();
    part_one(&mut temp_circuit);
    let actual_z_num = get_register_number(circuit, 'z');

    // Find the z-registers that don't match
    let mut mismatch_registers = expected_z_num
        .iter()
        .enumerate()
        .filter(|(i, bit)| *bit != &actual_z_num[*i])
        .map(|(i, _)| register_name('z', i))
        .collect::<HashSet<_>>();

    // Establish chain of custody. Any wire that affects a mismatched register is eligible for swapping.
    let mut chain_of_custody = HashSet::new();
    let mut processing_chain_of_custody = true;
    while processing_chain_of_custody {
        processing_chain_of_custody = false;
        for wire in temp_circuit.wires.iter() {
            if mismatch_registers.contains(&wire.output.name) && !chain_of_custody.contains(&wire) {
                mismatch_registers.insert(wire.left.name);
                mismatch_registers.insert(wire.right.name);
                chain_of_custody.insert(wire);
                processing_chain_of_custody = true;
            }
        }
    }

    // Now we know all the wires that might be crossed from top to bottom. We can try searching over 

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_circuit() {
        let circuit = read_input("evaluate.txt");
        let result = evaluate_circuit(&circuit);
        assert_eq!(result, 2024);
    }

    #[test]
    fn test_part_one() {
        let mut circuit = read_input("test.txt");
        let result = part_one(&mut circuit);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_bits() {
        let result = to_binary(4);
        //                       add_1  add_2  add_4
        assert_eq!(result, vec![false, false, true]);
    }
}

fn main() {
    let mut circuit = read_input("input.txt");
    let result = part_one(&mut circuit);
    println!("Part one: {}", result);
}
