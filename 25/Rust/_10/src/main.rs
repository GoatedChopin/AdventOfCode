use std::{collections::VecDeque, fs};

use num::Rational64;

mod solver;

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

// Helper function to check if all solution values are integers
fn is_integer_solution(solution: &Vec<Rational64>) -> bool {
    solution.iter().all(|x| *x.denom() == 1)
}

// Helper function to verify the solution produces the correct joltages
fn verify_joltages(solution: &Vec<Rational64>, machine: &Machine) -> bool {
    let mut joltage_state = vec![0i64; machine.joltages.len()];
    for (j, _jolt) in machine.joltages.iter().enumerate() {
        for (b, button) in machine.buttons.iter().enumerate() {
            if button & (1 << j) != 0 {
                joltage_state[j] += solution[b].numer();
            }
        }
    }
    joltage_state
        .iter()
        .zip(machine.joltages.iter())
        .all(|(x, y)| *x == *y as i64)
}

// Helper function to find the first fractional variable
fn find_fractional_variable(solution: &Vec<Rational64>) -> Option<usize> {
    solution.iter().position(|x| *x.denom() != 1)
}

// Helper function to find integer solution with branch-and-bound given an upper bound
fn find_integer_solution_with_bound(
    base_constraints: &Vec<(Vec<Rational64>, solver::Constraint, Rational64)>,
    num_buttons: usize,
    objective: &Vec<Rational64>,
    upper_bound: i64,
    machine: &Machine,
) -> Option<(Vec<Rational64>, i64)> {
    let mut constraints = base_constraints.clone();
    constraints.push((
        vec![Rational64::from(1); num_buttons],
        solver::Constraint::LessThan,
        Rational64::from(upper_bound),
    ));
    
    let solver = solver::SimplexSolver::new(constraints.clone(), num_buttons);
    let solution_result = solver.solve(objective);
    
    match solution_result {
        Ok(solution) => {
            if is_integer_solution(&solution) && verify_joltages(&solution, machine) {
                let value = solution.iter().map(|x| x.numer()).sum::<i64>();
                return Some((solution, value));
            }
            
            // Fractional solution - need to branch
            // Use simple branch-and-bound with limited depth
            let mut work_queue = VecDeque::new();
            work_queue.push_back(constraints);
            
            let mut best_solution: Option<(Vec<Rational64>, i64)> = None;
            let max_branches = 1000;
            let mut branches = 0;
            
            while let Some(branch_constraints) = work_queue.pop_front() {
                branches += 1;
                if branches > max_branches {
                    break;
                }
                
                let solver = solver::SimplexSolver::new(branch_constraints.clone(), num_buttons);
                if let Ok(sol) = solver.solve(objective) {
                    if is_integer_solution(&sol) && verify_joltages(&sol, machine) {
                        let val = sol.iter().map(|x| x.numer()).sum::<i64>();
                        if best_solution.is_none() || val < best_solution.as_ref().unwrap().1 {
                            best_solution = Some((sol, val));
                        }
                    } else if let Some(frac_idx) = find_fractional_variable(&sol) {
                        let frac_value = sol[frac_idx];
                        let floor_val = frac_value.numer() / frac_value.denom();
                        let ceil_val = if frac_value.numer() % frac_value.denom() == 0 {
                            floor_val
                        } else {
                            floor_val + 1
                        };
                        
                        let mut constraint_vec = vec![Rational64::ZERO; num_buttons];
                        constraint_vec[frac_idx] = Rational64::from(1);
                        
                        let mut left = branch_constraints.clone();
                        left.push((constraint_vec.clone(), solver::Constraint::LessThan, Rational64::from(floor_val)));
                        work_queue.push_back(left);
                        
                        let mut right = branch_constraints.clone();
                        right.push((constraint_vec, solver::Constraint::GreaterThan, Rational64::from(ceil_val)));
                        work_queue.push_back(right);
                    }
                }
            }
            
            best_solution
        }
        Err(_) => None,
    }
}

fn part_two(machines: &Vec<Machine>) -> i64 {
    let mut all_joltages = 0;
    for (m, machine) in machines.iter().enumerate() {
        println!(
            "Solving machine {} -> {:?}",
            m + 1,
            machine.joltages
        );
        
        // Set up base constraints: joltage equations + non-negativity
        let mut base_constraints: Vec<(Vec<Rational64>, solver::Constraint, Rational64)> = vec![];
        
        // Joltage equation constraints
        for (j, jolt) in machine.joltages.iter().enumerate() {
            let mut joltage_vector = vec![Rational64::ZERO; machine.buttons.len()];
            for (b, button) in machine.buttons.iter().enumerate() {
                if button & (1 << j) != 0 {
                    joltage_vector[b] = Rational64::from(1);
                }
            }
            base_constraints.push((
                joltage_vector,
                solver::Constraint::EqualTo,
                Rational64::from(*jolt as i64),
            ));
        }
        
        // Non-negativity constraints
        for b in 0..machine.buttons.len() {
            let mut button_vector = vec![Rational64::ZERO; machine.buttons.len()];
            button_vector[b] = Rational64::from(1);
            base_constraints.push((
                button_vector,
                solver::Constraint::GreaterThan,
                Rational64::ZERO,
            ));
        }
        
        // Binary search for minimum total button presses
        // Upper bound: sum of all joltages (worst case: each joltage needs 1 press)
        let upper_bound: i64 = machine.joltages.iter().map(|j| *j as i64).sum();
        let mut low = 0i64;
        let mut high = upper_bound;
        let mut best_solution: Option<Vec<Rational64>> = None;
        let mut best_value: Option<i64> = None;
        
        // Objective: minimize sum of button presses
        let objective = vec![Rational64::from(1); machine.buttons.len()];
        
        println!("  Binary searching in range [{}..{}]", low, high);
        
        while low <= high {
            let mid = (low + high) / 2;
            
            // Try to find an integer solution with sum <= mid using branch-and-bound if needed
            let result = find_integer_solution_with_bound(&base_constraints, machine.buttons.len(), &objective, mid, machine);
            
            match result {
                Some((solution, value)) => {
                    println!("    Found valid solution at mid={}: sum={}", mid, value);
                    best_solution = Some(solution);
                    best_value = Some(value);
                    high = value - 1;  // Try to find better
                }
                None => {
                    // No solution at this bound, need higher
                    println!("    Mid={}: no integer solution found", mid);
                    low = mid + 1;
                }
            }
        }
        
        if best_solution.is_none() {
            panic!("No solution found for machine {}", m + 1);
        }
        
        let total_presses = best_value.unwrap();
        println!("  Best solution: {} presses\n", total_presses);
        
        all_joltages += total_presses;
    }
    all_joltages
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
