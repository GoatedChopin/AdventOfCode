use num::rational::Rational64;

const ZERO: Rational64 = Rational64::ZERO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    LessThan,
    GreaterThan,
    EqualTo,
}

/// A minimal, working two-phase simplex solver with explicit basis tracking.
pub struct SimplexSolver {
    tableau: Vec<Vec<Rational64>>,
    basis: Vec<usize>,
    num_constraints: usize,
    num_decision: usize,
    slack_start: usize,
    artificial_start: usize,
    num_cols: usize,
}

impl SimplexSolver {
    /// Create a new simplex solver.
    /// 
    /// Arguments:
    /// - constraints: Vec of (coefficients, constraint_type, rhs)
    /// - num_decision: Number of decision variables
    pub fn new(
        constraints: Vec<(Vec<Rational64>, Constraint, Rational64)>,
        num_decision: usize,
    ) -> Self {
        let num_constraints = constraints.len();
        
        // Count slack and artificial variables
        let num_slack = constraints
            .iter()
            .filter(|(_, c, _)| matches!(c, Constraint::LessThan | Constraint::GreaterThan))
            .count();
        let num_artificial = constraints
            .iter()
            .filter(|(_, c, _)| matches!(c, Constraint::GreaterThan | Constraint::EqualTo))
            .count();

        let slack_start = num_decision;
        let artificial_start = slack_start + num_slack;
        let num_cols = artificial_start + num_artificial + 1; // +1 for RHS

        // Initialize tableau: constraints + objective row
        let mut tableau = vec![vec![ZERO; num_cols]; num_constraints + 1];
        let mut basis = vec![0; num_constraints];
        
        let mut slack_idx = 0;
        let mut artificial_idx = 0;

        // Fill constraint rows
        for (row, (coeffs, constraint_type, rhs)) in constraints.iter().enumerate() {
            // Decision variable coefficients
            for (col, &coeff) in coeffs.iter().enumerate() {
                tableau[row][col] = coeff;
            }

            // Add slack/artificial variables and set up initial basis
            match constraint_type {
                Constraint::LessThan => {
                    // x + s = b, s is basic
                    tableau[row][slack_start + slack_idx] = Rational64::from(1);
                    basis[row] = slack_start + slack_idx;
                    slack_idx += 1;
                }
                Constraint::GreaterThan => {
                    // x - s + a = b, a is basic
                    tableau[row][slack_start + slack_idx] = Rational64::from(-1);
                    tableau[row][artificial_start + artificial_idx] = Rational64::from(1);
                    basis[row] = artificial_start + artificial_idx;
                    slack_idx += 1;
                    artificial_idx += 1;
                }
                Constraint::EqualTo => {
                    // x + a = b, a is basic
                    tableau[row][artificial_start + artificial_idx] = Rational64::from(1);
                    basis[row] = artificial_start + artificial_idx;
                    artificial_idx += 1;
                }
            }

            // RHS
            tableau[row][num_cols - 1] = *rhs;
        }

        Self {
            tableau,
            basis,
            num_constraints,
            num_decision,
            slack_start,
            artificial_start,
            num_cols,
        }
    }

    /// Perform pivot operation and update basis.
    fn pivot(&mut self, pivot_row: usize, pivot_col: usize) {
        let pivot_element = self.tableau[pivot_row][pivot_col];
        
        // Normalize pivot row
        for col in 0..self.num_cols {
            self.tableau[pivot_row][col] = self.tableau[pivot_row][col] / pivot_element;
        }

        // Eliminate pivot column from other rows
        for row in 0..self.tableau.len() {
            if row == pivot_row {
                continue;
            }
            let factor = self.tableau[row][pivot_col];
            if factor == ZERO {
                continue;
            }
            for col in 0..self.num_cols {
                self.tableau[row][col] =
                    self.tableau[row][col] - factor * self.tableau[pivot_row][col];
            }
        }

        // Update basis
        self.basis[pivot_row] = pivot_col;
    }

    /// Find entering variable (most negative reduced cost, not in basis).
    fn find_entering(&self, max_col: usize) -> Option<usize> {
        let obj_row = self.num_constraints;
        let mut best_col = None;
        let mut most_negative = ZERO;

        for col in 0..max_col {
            // Skip if in basis
            if self.basis.contains(&col) {
                continue;
            }
            let reduced_cost = self.tableau[obj_row][col];
            if reduced_cost < most_negative {
                most_negative = reduced_cost;
                best_col = Some(col);
            }
        }

        best_col
    }

    /// Find leaving variable using minimum ratio test.
    fn find_leaving(&self, entering_col: usize) -> Option<usize> {
        let rhs_col = self.num_cols - 1;
        let mut best_row = None;
        let mut min_ratio = None;

        for row in 0..self.num_constraints {
            let coeff = self.tableau[row][entering_col];
            if coeff > ZERO {
                let ratio = self.tableau[row][rhs_col] / coeff;
                if min_ratio.is_none() || ratio < min_ratio.unwrap() {
                    min_ratio = Some(ratio);
                    best_row = Some(row);
                }
            }
        }

        best_row
    }

    /// Phase 1: Minimize sum of artificial variables.
    fn phase1(&mut self) -> Result<(), String> {
        let obj_row = self.num_constraints;
        let rhs_col = self.num_cols - 1;

        // Set up Phase 1 objective: minimize sum of artificial variables
        for col in 0..self.num_cols {
            self.tableau[obj_row][col] = ZERO;
        }
        for col in self.artificial_start..(rhs_col) {
            self.tableau[obj_row][col] = Rational64::from(1);
        }

        // Eliminate basic artificial variables from objective row
        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col >= self.artificial_start {
                // This is an artificial variable, eliminate it
                let factor = self.tableau[obj_row][basic_col];
                for col in 0..self.num_cols {
                    self.tableau[obj_row][col] =
                        self.tableau[obj_row][col] - factor * self.tableau[row][col];
                }
            }
        }

        // Simplex iterations
        for _ in 0..1000 {
            let entering = self.find_entering(rhs_col);
            if entering.is_none() {
                break; // Optimal
            }
            let entering_col = entering.unwrap();
            let leaving = self.find_leaving(entering_col);
            if leaving.is_none() {
                return Err("Phase 1 unbounded".to_string());
            }
            self.pivot(leaving.unwrap(), entering_col);
        }

        // Check if feasible
        if self.tableau[obj_row][rhs_col] > ZERO {
            return Err("Problem is infeasible".to_string());
        }

        Ok(())
    }

    /// Phase 2: Optimize original objective.
    fn phase2(&mut self, objective: &[Rational64]) -> Result<Vec<Rational64>, String> {
        let obj_row = self.num_constraints;
        let rhs_col = self.num_cols - 1;

        // Set up original objective (negated for minimization)
        for col in 0..self.num_cols {
            self.tableau[obj_row][col] = ZERO;
        }
        for col in 0..self.num_decision {
            self.tableau[obj_row][col] = -objective[col];
        }

        // Eliminate basic variables from objective row
        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col < self.num_decision {
                // Decision variable is basic, eliminate it
                let factor = self.tableau[obj_row][basic_col];
                for col in 0..self.num_cols {
                    self.tableau[obj_row][col] =
                        self.tableau[obj_row][col] - factor * self.tableau[row][col];
                }
            }
        }

        // Simplex iterations (only consider decision variables)
        for _ in 0..1000 {
            let entering = self.find_entering(self.slack_start); // Only decision variables
            if entering.is_none() {
                break; // Optimal
            }
            let entering_col = entering.unwrap();
            let leaving = self.find_leaving(entering_col);
            if leaving.is_none() {
                return Err("Phase 2 unbounded".to_string());
            }
            self.pivot(leaving.unwrap(), entering_col);
        }

        // Extract solution
        let mut solution = vec![ZERO; self.num_decision];
        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col < self.num_decision {
                solution[basic_col] = self.tableau[row][rhs_col];
            }
        }

        Ok(solution)
    }

    /// Solve the linear program.
    pub fn solve(mut self, objective: &[Rational64]) -> Result<Vec<Rational64>, String> {
        // Check if we have artificial variables
        if self.artificial_start < self.num_cols - 1 {
            self.phase1()?;
        }
        self.phase2(objective)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        // Same test case as simplex.rs:
        // Minimize x₁ + x₂ + x₃
        // Subject to:
        //   x₁ ≤ 10
        //   x₁ ≥ 8
        //   x₂ ≥ 20
        //   x₃ = 30
        
        let constraints = vec![
            (vec![1.into(), 0.into(), 0.into()], Constraint::LessThan, 10.into()),
            (vec![1.into(), 0.into(), 0.into()], Constraint::GreaterThan, 8.into()),
            (vec![0.into(), 1.into(), 0.into()], Constraint::GreaterThan, 20.into()),
            (vec![0.into(), 0.into(), 1.into()], Constraint::EqualTo, 30.into()),
        ];

        let solver = SimplexSolver::new(constraints, 3);
        let objective = vec![1.into(), 1.into(), 1.into()];
        let solution = solver.solve(&objective).unwrap();

        println!("Solution: {:?}", solution);
        
        // Expected: x₁ = 8, x₂ = 20, x₃ = 30 (minimum at lower bounds)
        assert_eq!(solution[0], 8.into());
        assert_eq!(solution[1], 20.into());
        assert_eq!(solution[2], 30.into());
    }
}

