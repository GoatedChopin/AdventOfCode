use num::rational::Rational64;

const ZERO: Rational64 = Rational64::ZERO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    LessThan,
    GreaterThan,
    EqualTo,
}

#[derive(Debug, Clone)]
pub struct SimplexSolver {
    constraints: Vec<(Vec<Rational64>, Constraint, Rational64)>,
    tableau: Vec<Vec<Rational64>>,
    basis: Vec<usize>,
    num_constraints: usize,
    num_decision: usize,
    slack_start: usize,
    artificial_start: usize,
    num_cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchDirection {
    Up,
    Down,
    Stay,
}

impl SimplexSolver {
    pub fn new(
        constraints: Vec<(Vec<Rational64>, Constraint, Rational64)>,
        num_decision: usize,
    ) -> Self {
        let num_constraints = constraints.len();

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
        let num_cols = artificial_start + num_artificial + 1;

        let mut tableau = vec![vec![ZERO; num_cols]; num_constraints + 1];
        let mut basis = vec![0; num_constraints];

        let mut slack_idx = 0;
        let mut artificial_idx = 0;

        for (row, (coeffs, constraint_type, rhs)) in constraints.iter().enumerate() {
            for (col, &coeff) in coeffs.iter().enumerate() {
                tableau[row][col] = coeff;
            }

            match constraint_type {
                Constraint::LessThan => {
                    tableau[row][slack_start + slack_idx] = Rational64::from(1);
                    basis[row] = slack_start + slack_idx;
                    slack_idx += 1;
                }
                Constraint::GreaterThan => {
                    tableau[row][slack_start + slack_idx] = Rational64::from(-1);
                    tableau[row][artificial_start + artificial_idx] = Rational64::from(1);
                    basis[row] = artificial_start + artificial_idx;
                    slack_idx += 1;
                    artificial_idx += 1;
                }
                Constraint::EqualTo => {
                    tableau[row][artificial_start + artificial_idx] = Rational64::from(1);
                    basis[row] = artificial_start + artificial_idx;
                    artificial_idx += 1;
                }
            }

            tableau[row][num_cols - 1] = *rhs;
        }

        Self {
            constraints,
            tableau,
            basis,
            num_constraints,
            num_decision,
            slack_start,
            artificial_start,
            num_cols,
        }
    }

    fn pivot(&mut self, pivot_row: usize, pivot_col: usize) {
        let pivot_element = self.tableau[pivot_row][pivot_col];

        for col in 0..self.num_cols {
            self.tableau[pivot_row][col] = self.tableau[pivot_row][col] / pivot_element;
        }

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

        self.basis[pivot_row] = pivot_col;
    }

    fn find_entering(&self, max_col: usize) -> Option<usize> {
        let obj_row = self.num_constraints;
        let mut best_col = None;
        let mut most_negative = ZERO;

        for col in 0..max_col {
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

    fn phase1(&mut self) -> Option<Vec<Rational64>> {
        let obj_row = self.num_constraints;
        let rhs_col = self.num_cols - 1;

        for col in 0..self.num_cols {
            self.tableau[obj_row][col] = ZERO;
        }
        for col in self.artificial_start..(rhs_col) {
            self.tableau[obj_row][col] = Rational64::from(1);
        }

        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col >= self.artificial_start {
                let factor = self.tableau[obj_row][basic_col];
                for col in 0..self.num_cols {
                    self.tableau[obj_row][col] =
                        self.tableau[obj_row][col] - factor * self.tableau[row][col];
                }
            }
        }

        for _ in 0..1000 {
            let entering = self.find_entering(rhs_col);
            if entering.is_none() {
                break;
            }
            let entering_col = entering.unwrap();
            let leaving = self.find_leaving(entering_col);
            if leaving.is_none() {
                return None;
            }
            self.pivot(leaving.unwrap(), entering_col);
        }

        if self.tableau[obj_row][rhs_col] > ZERO {
            return None;
        }

        Some(vec![])
    }

    fn phase2(&mut self, objective: &[Rational64]) -> Option<Vec<Rational64>> {
        let obj_row = self.num_constraints;
        let rhs_col = self.num_cols - 1;

        for col in 0..self.num_cols {
            self.tableau[obj_row][col] = ZERO;
        }
        for col in 0..self.num_decision {
            self.tableau[obj_row][col] = -objective[col];
        }

        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col < self.num_decision {
                let factor = self.tableau[obj_row][basic_col];
                for col in 0..self.num_cols {
                    self.tableau[obj_row][col] =
                        self.tableau[obj_row][col] - factor * self.tableau[row][col];
                }
            }
        }

        for _ in 0..1000 {
            let entering = self.find_entering(self.slack_start);
            if entering.is_none() {
                break;
            }
            let entering_col = entering.unwrap();
            let leaving = self.find_leaving(entering_col);
            if leaving.is_none() {
                return None;
            }
            self.pivot(leaving.unwrap(), entering_col);
        }

        let mut solution = vec![ZERO; self.num_decision];
        for row in 0..self.num_constraints {
            let basic_col = self.basis[row];
            if basic_col < self.num_decision {
                solution[basic_col] = self.tableau[row][rhs_col];
            }
        }

        Some(solution)
    }

    pub fn solve(mut self, objective: &[Rational64]) -> Option<Vec<Rational64>> {
        if self.artificial_start < self.num_cols - 1 {
            let is_feasible= self.phase1();
            if is_feasible.is_none() {
                return None;
            }
        }
        self.phase2(objective)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver() {
        let constraints = vec![
            (
                vec![1.into(), 0.into(), 0.into()],
                Constraint::LessThan,
                10.into(),
            ),
            (
                vec![1.into(), 0.into(), 0.into()],
                Constraint::GreaterThan,
                8.into(),
            ),
            (
                vec![0.into(), 1.into(), 0.into()],
                Constraint::GreaterThan,
                20.into(),
            ),
            (
                vec![0.into(), 0.into(), 1.into()],
                Constraint::EqualTo,
                30.into(),
            ),
        ];

        let solver = SimplexSolver::new(constraints, 3);
        let objective = vec![1.into(), 1.into(), 1.into()];
        let solution = solver.solve(&objective).unwrap();

        println!("Solution: {:?}", solution);

        assert_eq!(solution[0], 8.into());
        assert_eq!(solution[1], 20.into());
        assert_eq!(solution[2], 30.into());
    }
}
