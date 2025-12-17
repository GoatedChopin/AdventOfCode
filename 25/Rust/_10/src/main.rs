use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fs,
    os::unix::raw::blkcnt_t,
};

mod lp;
use lp::{Matrix, search_constraints};

#[derive(Debug, Clone)]
struct Machine {
    desired: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

#[derive(Debug, Clone)]
struct MachineState {
    current: u16,
    num_presses: u32,
}

#[derive(Debug, Clone)]
struct JoltageState {
    current: Vec<u16>,
    num_presses: u32,
}

enum ParseState {
    Desired,
    Buttons,
    Dimensions,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let mut state = ParseState::Desired;
        let parts = s.split(" ").collect::<Vec<&str>>();
        let mut desired = 0;
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            match state {
                ParseState::Desired => {
                    let desired_unformatted = part;
                    for (i, c) in desired_unformatted
                        .replace("[", "")
                        .replace("]", "")
                        .chars()
                        .enumerate()
                    {
                        if c == '#' {
                            desired |= 1 << i;
                        }
                    }
                    state = ParseState::Buttons;
                }
                ParseState::Buttons => {
                    if i == parts.len() - 2 {
                        state = ParseState::Dimensions;
                    }
                    let flip_bits: Vec<u16> = part
                        .replace("(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();
                    let mut button = 0;
                    for bit in flip_bits {
                        button |= 1 << bit;
                    }
                    buttons.push(button);
                }
                ParseState::Dimensions => {
                    joltages = part
                        .replace("{", "")
                        .replace("}", "")
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();
                }
            }
        }
        Self {
            desired,
            buttons,
            joltages,
        }
    }
}

fn read_input(path: &str) -> Vec<Machine> {
    let lines = fs::read_to_string(path).expect("Failed to read input file");
    lines.lines().map(|line| Machine::from_str(line)).collect()
}

fn minimum_presses(machine: &Machine) -> u32 {
    let presses = 0;
    let mut queue = VecDeque::new();
    queue.push_back(MachineState {
        current: 0,
        num_presses: 0,
    });
    while !queue.is_empty() {
        let m_state = queue.pop_front().unwrap();
        // println!("Current: {:08b}, want {:08b}", m_state.current, machine.desired);
        if m_state.current == machine.desired {
            return m_state.num_presses;
        }
        for button in &machine.buttons {
            let mut new_machine = m_state.clone();
            new_machine.current ^= button;
            new_machine.num_presses += 1;
            queue.push_back(new_machine);
        }
    }
    presses
}

fn part_one(machines: &Vec<Machine>) -> u32 {
    let mut all_presses = 0;
    for machine in machines {
        // machine.print();
        all_presses += minimum_presses(machine);
    }
    all_presses
}

fn part_two(machines: &Vec<Machine>) -> u32 {
    let mut all_joltages = 0;
    for machine in machines {
        // Each machine needs a matrix of m x n where m is the machine
        let m = machine.joltages.iter().enumerate().map(|(j, jolt)| {
            let mut row = vec![0.0; machine.buttons.len() + 1];
            for (b, button) in machine.buttons.iter().enumerate() {
                if button & (1 << j) != 0 {
                    row[b] = 1.0;
                }
            }
            row[machine.buttons.len()] = *jolt as f64;
            row
        }).collect();
        let grid = Matrix::from_data(m);
        // println!("Grid:");
        // grid.print();
        let reduced_grid = grid.gaussian_form();
        // println!("Reduced grid:");
        // reduced_grid.print();
        let constraints = reduced_grid.get_constraints();

        for (i, constraint) in constraints.iter().enumerate() {
            println!("  Constraint: {:.2} <= x{} <= {:.2}", constraint.lower_bound, i, constraint.upper_bound);
        }

        let coefficients = search_constraints(
            &grid
                .data
                .clone()
                .into_iter()
                .map(|row| {
                    row[..row.len() - 1]
                        .into_iter()
                        .map(|x| x.round() as i32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            &constraints,
            &machine.joltages.iter().map(|j| *j as i32).collect(),
        );
        all_joltages += coefficients.iter().map(|x| *x as u32).sum::<u32>();
    }
    all_joltages as u32
}

fn test() {
    let machine = Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    assert_eq!(machine.desired, 0b00110);
    let input = read_input("test.txt");
    assert_eq!(part_one(&input), 7);
    assert_eq!(part_two(&input), 33);
}

fn main() {
    test();
    let input = read_input("input.txt");
    // println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
