use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

fn register_to_string(r: &[char; 3]) -> String {
    r.iter().collect()
}

fn make_key(a: &[char; 3], b: &[char; 3], op: Operation) -> (String, String, Operation) {
    let a_str: String = a.iter().collect();
    let b_str: String = b.iter().collect();
    if a_str < b_str {
        (a_str, b_str, op)
    } else {
        (b_str, a_str, op)
    }
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

fn ahead_of_time_checker(circuit: &Circuit) -> HashSet<String> {
    // Build gate map: (Input1, Input2, Op) -> OutputWire
    let mut gate_map: HashMap<(String, String, Operation), String> = HashMap::new();
    for wire in &circuit.wires {
        let key = make_key(&wire.left.name, &wire.right.name, wire.operation);
        gate_map.insert(key, register_to_string(&wire.output.name));
    }

    // Derive bit count from circuit (max z-index + 1)
    let num_bits = circuit
        .registers
        .keys()
        .filter(|r| r[0] == 'z')
        .map(|r| format!("{}{}", r[1], r[2]).parse::<usize>().unwrap_or(0))
        .max()
        .map(|i| i + 1)
        .unwrap_or(45);

    let mut swaps: HashSet<String> = HashSet::new();
    let mut c_in: Option<String> = None;

    for i in 0..num_bits {
        let x = register_to_string(&register_name('x', i));
        let y = register_to_string(&register_name('y', i));
        let z = register_to_string(&register_name('z', i));

        let mut xor1 = gate_map.get(&make_key(&register_name('x', i), &register_name('y', i), Operation::XOR)).cloned();
        let mut and1 = gate_map.get(&make_key(&register_name('x', i), &register_name('y', i), Operation::AND)).cloned();

        if i == 0 {
            // Half Adder: z00 must be x00 XOR y00
            if let Some(ref out) = xor1 {
                if out != &z {
                    swaps.insert(out.clone());
                    swaps.insert(z.clone());
                }
            } else {
                // No XOR gate for x,y at bit 0 - z is wrong
                swaps.insert(z.clone());
            }
            c_in = and1;
            continue;
        }

        let carry = match &c_in {
            Some(c) => c.clone(),
            None => continue,
        };

        // Find the gate that outputs to z
        let z_gate_key = gate_map
            .iter()
            .find(|(_, out)| **out == z)
            .map(|(k, _)| (k.0.clone(), k.1.clone(), k.2));

        if let Some((in_a, in_b, op)) = z_gate_key {
            if op != Operation::XOR {
                swaps.insert(z.clone());
            } else {
                let is_connected_to_carry = in_a == carry || in_b == carry;

                if let Some(ref x1) = xor1 {
                    if in_a != carry && in_a != *x1 {
                        swaps.insert(x1.clone());
                        swaps.insert(in_a.clone());
                        xor1 = Some(in_a.clone());
                    } else if in_b != carry && in_b != *x1 {
                        swaps.insert(x1.clone());
                        swaps.insert(in_b.clone());
                        xor1 = Some(in_b.clone());
                    }
                }
            }
        } else {
            swaps.insert(z.clone());
        }

        // Re-check Z output
        if let Some(ref x1) = xor1 {
            let z_expect = gate_map.get(&make_key_str(x1, &carry, Operation::XOR));
            if let Some(out) = z_expect {
                if out != &z {
                    swaps.insert(out.clone());
                    swaps.insert(z.clone());
                }
            }
        }

        // Next carry: (xor1 AND carry) OR and1
        if let Some(ref x1) = xor1 {
            let and2 = gate_map.get(&make_key_str(x1, &carry, Operation::AND));

            if let Some(a2) = and2 {
                if let Some(ref a1) = and1 {
                    let next_carry_gate = gate_map.get(&make_key_str(a1, a2, Operation::OR));
                    c_in = next_carry_gate.cloned();
                }
            }
        }
    }

    swaps
}

fn make_key_str(a: &str, b: &str, op: Operation) -> (String, String, Operation) {
    if a < b {
        (a.to_string(), b.to_string(), op)
    } else {
        (b.to_string(), a.to_string(), op)
    }
}

fn part_two(circuit: &mut Circuit) -> String {
    let swaps = ahead_of_time_checker(circuit);
    let mut result: Vec<String> = swaps.into_iter().collect();
    result.sort();
    result.join(",")
}

#[allow(dead_code)]
fn get_required_values(
    wire: &Wire,
    viable_values: &HashMap<&[char; 3], HashSet<bool>>,
) -> (Option<bool>, Option<bool>) {
    let output_values = viable_values.get(&wire.output.name).unwrap();
    if output_values.len() > 1 {
        return (None, None);
    }
    let left_values = viable_values.get(&wire.left.name).unwrap();
    let right_values = viable_values.get(&wire.right.name).unwrap();
    let left_value = if left_values.len() == 1 {
        left_values.iter().next()
    } else {
        None
    };
    let right_value = if right_values.len() == 1 {
        right_values.iter().next()
    } else {
        None
    };
    let required_output_result = output_values.iter().next();
    if required_output_result.is_none() {
        return (None, None);
    }
    let required_output = required_output_result.unwrap();
    match wire.operation {
        Operation::AND => {
            if !required_output {
                return (None, None);
            }
            return (Some(true), Some(true));
        }
        Operation::OR => {
            if *required_output {
                return (None, None);
            }
            return (Some(false), Some(false));
        }
        Operation::XOR => {
            let mut req = (None, None);
            if left_value.is_none() && right_value.is_none() {
                return req;
            }
            req.0 = if left_value.is_some() {
                Some(*left_value.unwrap())
            } else {
                None
            };
            req.1 = if right_value.is_some() {
                Some(*right_value.unwrap())
            } else {
                None
            };
            let non_null = if req.0.is_some() {
                req.0.unwrap()
            } else {
                req.1.unwrap()
            };
            match required_output {
                true => {
                    return (
                        Some(req.0.unwrap_or(!non_null)),
                        Some(!req.1.unwrap_or(!non_null)),
                    );
                }
                false => {
                    return (
                        Some(!req.0.unwrap_or(non_null)),
                        Some(!req.1.unwrap_or(non_null)),
                    );
                }
            }
        }
    }
}

#[allow(dead_code)]
fn recurse_viable_values(
    circuit: &Circuit,
    register: [char; 3],
    viable_values: &mut HashMap<&[char; 3], HashSet<bool>>,
) {
    for wire in circuit.wires.iter() {
        if wire.output.name == register {
            let (left_value, right_value) = get_required_values(wire, viable_values);
            if left_value.is_some() {
                viable_values
                    .get_mut(&wire.left.name)
                    .unwrap()
                    .remove(&!left_value.unwrap());
                recurse_viable_values(circuit, wire.left.name, viable_values);
            }
            if right_value.is_some() {
                viable_values
                    .get_mut(&wire.right.name)
                    .unwrap()
                    .remove(&!right_value.unwrap());
                recurse_viable_values(circuit, wire.right.name, viable_values);
            }
        }
    }
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
    let result = part_two(&mut circuit);
    println!("Part two: {}", result);
}
