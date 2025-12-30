use num::rational::Rational64;
use std::fs;

mod solver;
use solver::{Constraint, SimplexSolver};

struct ClawMachine {
    prices: Vec<Rational64>,
    buttons: Vec<Vec<Rational64>>,
    objective: Vec<Rational64>,
}

impl ClawMachine {
    pub fn constraints(&self) -> Vec<(Vec<Rational64>, Constraint, Rational64)> {
        let mut constraints = Vec::new();
        for (b, button) in self.buttons.iter().enumerate() {
            let mut coefficients = vec![Rational64::ZERO; self.buttons.len()];
            coefficients[b] = Rational64::from(1);

            // lower_bound
            constraints.push((
                coefficients.clone(),
                Constraint::GreaterThan,
                Rational64::from(0),
            ));
            // upper_bounds
            for (o, objective) in self.objective.iter().enumerate() {
                let ratio = objective / button[o];
                constraints.push((coefficients.clone(), Constraint::LessThan, ratio));
            }
        }
        for (o, objective) in self.objective.iter().enumerate() {
            // Add an equality constraint for the objective
            let mut coefficients = vec![Rational64::ZERO; self.buttons.len()];
            for (b, button) in self.buttons.iter().enumerate() {
                coefficients[b] = button[o];
            }
            constraints.push((coefficients, Constraint::EqualTo, *objective));
        }
        constraints
    }

    pub fn scoring_fn(&self, solution: &Vec<Rational64>) -> Rational64 {
        solution
            .iter()
            .enumerate()
            .map(|(i, x)| x * self.prices[i])
            .sum::<Rational64>()
    }

    pub fn valid_solution(&self, solution: &Vec<Rational64>, objective: &Vec<Rational64>) -> bool {
        if !solution
            .iter()
            .enumerate()
            .all(|(_i, x)| x.is_integer() && x >= &Rational64::from(0))
        {
            return false;
        }
        // Check that solution * self.buttons == objective
        let mut product = vec![Rational64::ZERO; objective.len()];
        for (b, presses) in solution.iter().enumerate() {
            let button = &self.buttons[b];
            for (v, value) in button.iter().enumerate() {
                product[v] += presses * value;
            }
        }
        product == *objective
    }

    pub fn solve(&self) -> Rational64 {
        let constraints = self.constraints();
        // Use Branch and Bound to find optimal integer solution
        self.branch_and_bound(&constraints)
    }

    fn branch_and_bound(&self, constraints: &Vec<(Vec<Rational64>, Constraint, Rational64)>) -> Rational64 {
        let mut best_cost = None;
        
        // Use Branch and Bound
        let mut stack: Vec<Vec<(Vec<Rational64>, Constraint, Rational64)>> = Vec::new();
        stack.push(constraints.clone());
        
        while let Some(current_constraints) = stack.pop() {
            // Solve LP relaxation for current node
            let solver = SimplexSolver::new(current_constraints.clone(), self.buttons.len());
            let solution = solver.solve(&self.prices);
            
            if solution.is_err() {
                // Infeasible, prune this branch
                continue;
            }
            
            let solution = solution.unwrap();
            let cost = self.scoring_fn(&solution);
            
            // Prune if this branch can't improve best solution
            if best_cost.is_some() && cost >= best_cost.unwrap() {
                continue;
            }
            
            // Check if solution is integer and valid
            if self.valid_solution(&solution, &self.objective) {
                // Found a valid integer solution
                if best_cost.is_none() || cost < best_cost.unwrap() {
                    best_cost = Some(cost);
                }
                continue;
            }
            
            // Find first non-integer variable to branch on
            let mut branch_var = None;
            for (i, value) in solution.iter().enumerate() {
                if !value.is_integer() {
                    branch_var = Some(i);
                    break;
                }
            }
            
            if let Some(var_idx) = branch_var {
                let value = solution[var_idx];
                let floor_val = value.floor();
                let ceil_val = value.ceil();
                
                // Left branch: x <= floor(value)
                let mut left_constraints = current_constraints.clone();
                let mut left_coeffs = vec![Rational64::ZERO; self.buttons.len()];
                left_coeffs[var_idx] = Rational64::from(1);
                left_constraints.push((left_coeffs, Constraint::LessThan, floor_val));
                stack.push(left_constraints);
                
                // Right branch: x >= ceil(value)
                let mut right_constraints = current_constraints.clone();
                let mut right_coeffs = vec![Rational64::ZERO; self.buttons.len()];
                right_coeffs[var_idx] = Rational64::from(1);
                right_constraints.push((right_coeffs, Constraint::GreaterThan, ceil_val));
                stack.push(right_constraints);
            }
        }
        
        if let Some(best) = best_cost {
            best
        } else {
            panic!("No integer solution found");
        }
    }
}

enum InputState {
    Buttons,
    Prize,
}

fn read_input(path: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    let binding = fs::read_to_string(path).expect("Failed to read file");
    let lines = binding.lines().collect::<Vec<&str>>();

    let mut state = InputState::Buttons;
    let mut buttons = Vec::new();
    for line in lines {
        if line.starts_with("Prize") {
            state = InputState::Prize;
        } else if line.is_empty() {
            state = InputState::Buttons;
            continue;
        }

        match state {
            InputState::Buttons => {
                let parts = line
                    .replace(",", "")
                    .replace("X+", "")
                    .replace("Y+", "")
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let x = parts[2].parse::<i64>().unwrap();
                let y = parts[3].parse::<i64>().unwrap();
                buttons.push(vec![Rational64::from(x), Rational64::from(y)]);
            }
            InputState::Prize => {
                let parts = line
                    .replace("Prize: X=", "")
                    .replace(" Y=", " ")
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let x = parts[0].parse::<i64>().unwrap();
                let y = parts[1].parse::<i64>().unwrap();
                machines.push(ClawMachine {
                    prices: vec![Rational64::from(3), Rational64::from(1)],
                    buttons: buttons.clone(),
                    objective: vec![Rational64::from(x), Rational64::from(y)],
                });
                buttons = Vec::new();
            }
        }
    }

    machines
}

fn part_one(machines: &Vec<ClawMachine>) -> Rational64 {
    let mut total_tokens = Rational64::ZERO;
    for machine in machines {
        let solution = machine.solve();
        total_tokens += solution;
    }
    total_tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let machines = read_input("test.txt");
        let solution = part_one(&machines);
        assert_eq!(solution, Rational64::from(28138));
    }
}

fn main() {
    let machines = read_input("input.txt");
    let solution = part_one(&machines);
    println!("Solution: {}", solution);
}
