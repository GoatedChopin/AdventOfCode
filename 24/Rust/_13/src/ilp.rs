use std::collections::HashSet;
use num::rational::Rational64;

use crate::solver::{Constraint, SimplexSolver};

const ZERO: Rational64 = Rational64::ZERO;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchAndBoundMode {
    Minimize,
    Maximize,
}

fn print_solution(solution: &Option<Vec<Rational64>>, objective: &Vec<Rational64>, mode: BranchAndBoundMode) {
    if solution.is_none() {
        return;
    }
    let solution = solution.as_ref().unwrap();
    println!("Solution for objective {:?} over {:?} is:", objective, mode);
    for (i, x) in solution.iter().enumerate() {
        println!("  {} = {}", i, x);
    }
}

pub fn compare_solutions(
    solution1: &Vec<Rational64>,
    solution2: &Vec<Rational64>,
    mode: BranchAndBoundMode,
) -> bool {
    match mode {
        BranchAndBoundMode::Minimize => {
            solution1.iter().sum::<Rational64>() < solution2.iter().sum::<Rational64>()
        }
        BranchAndBoundMode::Maximize => {
            solution1.iter().sum::<Rational64>() > solution2.iter().sum::<Rational64>()
        }
    }
}

pub fn branch_and_bound(
    constraints: &Vec<(Vec<Rational64>, Constraint, Rational64)>,
    objective: &Vec<Rational64>,
    mode: BranchAndBoundMode,
) -> Option<Vec<Rational64>> {
    let mut best_solution: Option<Vec<Rational64>> = None;
    let mut previous_branches = HashSet::new();
    let mut stack: Vec<Vec<(Vec<Rational64>, Constraint, Rational64)>> = Vec::new();
    stack.push(constraints.clone());

    // let mut depth = 0;
    while let Some(current_constraints) = stack.pop() {
        // if depth % 100 == 0 {
        //     println!("    Nodes remaining: {}", stack.len());
        // }
        // depth += 1;
        let solver = SimplexSolver::new(current_constraints.clone(), objective.len());
        let solution = solver.solve(objective);
        if solution.is_none() {
            // println!("    Pruning branch with no solution");
            continue;
        }
        let solution = solution.unwrap();
        // println!("      Solution: {:?}", solution);
        // println!("      Constraints: {:?}", current_constraints);
        // println!("      Objective: {:?}", objective);
        // println!("      Mode: {:?}", mode);
        if best_solution.is_some()
            && !compare_solutions(&solution, best_solution.as_ref().unwrap(), mode)
        {
            // println!("    Pruning branch with solution {:?}", solution);
            continue;
        }
        if solution.iter().all(|x| x.is_integer()) {
            // println!(
            //     "New best solution: {:?} for objective {:?}",
            //     solution, objective
            // );
            best_solution = Some(solution);
            continue;
        }
        for (i, x) in solution.iter().enumerate() {
            if x.is_integer() {
                continue;
            }
            let branch_key = (i, *x);
            if previous_branches.contains(&branch_key) {
                continue;
            }
            previous_branches.insert(branch_key);
            // println!("  Branching on variable {} with value {}", i, x);
            let mut new_constraints = current_constraints.clone();
            let mut branch_constraint = vec![Rational64::ZERO; objective.len()];
            branch_constraint[i] = Rational64::from(1);
            new_constraints.push((
                branch_constraint.clone(),
                Constraint::GreaterThan,
                solution[i].ceil(),
            ));
            stack.push(new_constraints);
            let mut new_constraints = current_constraints.clone();
            branch_constraint[i] = Rational64::from(1);
            new_constraints.push((branch_constraint, Constraint::LessThan, solution[i].floor()));
            stack.push(new_constraints);
            break;
        }
    }

    // print_solution(&best_solution, objective, mode);
    best_solution
}
