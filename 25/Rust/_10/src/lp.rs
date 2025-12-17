use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
};

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub free_variables: Vec<usize>,
    pub dependent_variables: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Constraint {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }
}

const EPSILON: f64 = 1e-9;

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            free_variables: Vec::new(),
            dependent_variables: Vec::new(),
        }
    }

    pub fn print(&self) {
        for row in &self.data {
            println!(
                "{}",
                row.iter()
                    .map(|x| format!("{:.2}", x))
                    .collect::<Vec<String>>()
                    .join("\t")
            );
        }
    }

    pub fn from_data(data: Vec<Vec<f64>>) -> Self {
        Self {
            data,
            free_variables: Vec::new(),
            dependent_variables: Vec::new(),
        }
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    pub fn gaussian_form(&self) -> Matrix {
        let mut matrix = self.clone();

        let mut pivot_row = 0;
        let mut pivot_val;
        let mut col = 0;
        while pivot_row < self.rows() && col < self.cols() {
            pivot_row = col;
            let row_result = matrix
                .data
                .iter()
                .enumerate()
                .skip(col)
                .map(|(row, row_data)| (row, row_data[col]))
                .max_by(|(_, a), (_, b)| {
                    if a.abs() < EPSILON || b.abs() < EPSILON {
                        return a.abs().partial_cmp(&b.abs()).unwrap();
                    } else if (a % b).abs() < EPSILON {
                        // If b fits evenly into a, then b is better.
                        return Ordering::Less;
                    } else if (b % a).abs() < EPSILON {
                        // If a fits evenly into b, then a is better.
                        return Ordering::Greater;
                    } else {
                        return b.abs().partial_cmp(&a.abs()).unwrap();
                    }
                });

            if row_result.is_none() {
                col += 1;
                matrix.free_variables.push(col);
                continue;
            }

            let (best_row, best_val) = row_result.unwrap();

            if best_val.abs() < EPSILON {
                col += 1;
                matrix.free_variables.push(col);
                continue;
            }

            pivot_row = best_row;
            pivot_val = best_val;

            matrix.dependent_variables.push(col);

            // Swap pivot row to the top.
            matrix.data.swap(col, pivot_row);
            pivot_row = col;
            matrix.data[pivot_row] = matrix.data[pivot_row]
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    if i < col {
                        return *val;
                    }
                    *val / pivot_val
                })
                .collect();

            // println!("Swapped rows {col} and {pivot_row}:");
            // matrix.print();

            // Eliminate this column in all other rows.
            for r in 0..self.rows() {
                if r > pivot_row {
                    let factor = matrix.data[r][col];
                    if factor.abs() > EPSILON {
                        let range = col..self.cols();
                        let pivot_row: Vec<f64> = matrix.data[pivot_row][range.clone()].to_vec();
                        matrix.data[r][range.clone()]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pv)| {
                                // println!("  {val} - {factor} * {pv} = {}", val.clone() - factor * pv);
                                *val -= factor * pv;
                            });
                    }
                }
            }
            // println!("After elimination, {col}:");
            // matrix.print();
            col += 1;
            pivot_row = col;
        }
        Self {
            data: matrix.data,
            free_variables: matrix.free_variables,
            dependent_variables: matrix.dependent_variables,
        }
    }

    pub fn get_constraints(&self) -> Vec<Constraint> {
        let mut constraints = vec![Constraint::new(0.0, f64::INFINITY); self.cols() - 1];
        let mut matrix = self.clone();

        let mut can_improve = true;

        while can_improve {
            can_improve = false;
            let mut matrix_data = matrix.data.clone();
            // For each row, if only 1 column (behind the final column) is non-zero, that is a known quantity
            for (r, row) in matrix.data.iter().enumerate() {
                let nonzero_cols = row
                    .iter()
                    .enumerate()
                    .filter(|(col, val)| val.abs() > EPSILON && *col < self.cols() - 1)
                    .collect::<Vec<_>>();

                if nonzero_cols.len() == 0 {
                    continue;
                }

                if nonzero_cols.len() == 1 {
                    let (col, _) = nonzero_cols[0];
                    constraints[col].lower_bound = row[self.cols() - 1];
                    constraints[col].upper_bound = row[self.cols() - 1];

                    // See if we can tighten the bounds of other variables in this same column
                    for (other_r, other_row) in matrix.data.iter().enumerate() {
                        if other_r == r {
                            continue;
                        }
                        if other_row[col].abs() <= EPSILON {
                            continue;
                        }
                        can_improve = true;

                        // Subtract the deterministic row from this row
                        let factor = other_row[col] / row[col];
                        for c in 0..self.cols() {
                            matrix_data[other_r][c] -= factor * row[c];
                        }
                    }
                    continue;
                }

                // There are multiple non-zero values, so this is a dependent variable.
                // The values of each nonzero column affects the value of the other nonzero columns.
                let rhs = row[self.cols() - 1];

                // Check every variable in this equation to see if we can tighten it
                for &(target_col, target_coeff) in &nonzero_cols {
                    let mut sum_min = 0.0;
                    let mut sum_max = 0.0;

                    // Calculate the range of the REST of the equation
                    for &(other_col, other_coeff) in &nonzero_cols {
                        if other_col == target_col {
                            continue;
                        }

                        let lb = constraints[other_col].lower_bound;
                        let ub = constraints[other_col].upper_bound;

                        // Logic to add to sum_min / sum_max safely handling Infinity
                        if *other_coeff > 0.0 {
                            sum_min += if lb == f64::NEG_INFINITY {
                                f64::NEG_INFINITY
                            } else {
                                other_coeff * lb
                            };
                            sum_max += if ub == f64::INFINITY {
                                f64::INFINITY
                            } else {
                                other_coeff * ub
                            };
                        } else {
                            // Negative coeff flips things: -2 * infinity = -infinity (which is the new min)
                            sum_min += if ub == f64::INFINITY {
                                f64::NEG_INFINITY
                            } else {
                                other_coeff * ub
                            };
                            sum_max += if lb == f64::NEG_INFINITY {
                                f64::INFINITY
                            } else {
                                other_coeff * lb
                            };
                        }
                    }

                    // Isolate target:  target_coeff * x = RHS - Sum
                    // So: x = (RHS - Sum) / target_coeff

                    // Calculate raw bounds
                    let val1 = (rhs - sum_min) / target_coeff;
                    let val2 = (rhs - sum_max) / target_coeff;

                    let (mut new_lb, mut new_ub) = if *target_coeff > 0.0 {
                        (val2, val1) // subtract max sum gives min result
                    } else {
                        (val1, val2) // dividing by negative swaps sides
                    };

                    // Apply Integer Logic (Ceil/Floor) + Clamp to 0 (non-negative constraint)
                    new_lb = new_lb.ceil().max(0.0);
                    new_ub = new_ub.floor();

                    // Update Constraints if tighter
                    let current = &mut constraints[target_col];

                    if new_lb > current.lower_bound + EPSILON {
                        current.lower_bound = new_lb;
                        can_improve = true;
                    }
                    if new_ub < current.upper_bound - EPSILON {
                        current.upper_bound = new_ub;
                        can_improve = true;
                    }

                    // Sanity Check: If bounds crossed, no solution exists for this branch
                    if current.lower_bound > current.upper_bound {
                        return vec![]; // Or handle "Impossible" error
                    }
                }
            }
            matrix.data = matrix_data;
        }

        return constraints;
    }
}

// Get all vertices of the polytope defined by the constraints
pub fn all_vertices(constraints: &[Constraint], current: Option<Vec<i32>>) -> Vec<Vec<i32>> {
    let current = current.unwrap_or(vec![]);
    
    // Base case: if no more constraints, return the current vertex
    if constraints.is_empty() {
        return vec![current];
    }

    let constraint = &constraints[0];
    let mut up = current.clone();
    up.push(constraint.upper_bound.round().max(0.0) as i32);
    let mut down = current.clone();
    down.push(constraint.lower_bound.round().max(0.0) as i32);
    
    let mut vertices = Vec::new();
    let remaining = &constraints[1..];
    vertices.extend(all_vertices(remaining, Some(down)));
    vertices.extend(all_vertices(remaining, Some(up)));

    vertices.sort_by(|a, b| a.iter().sum::<i32>().cmp(&b.iter().sum::<i32>()));
    vertices.dedup();
    vertices
}

struct CoefficientState {
    coefficients: Vec<i32>,
    distance: i32,
}

impl PartialOrd for CoefficientState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl Ord for CoefficientState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialEq for CoefficientState {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for CoefficientState {}

fn product(matrix: &Vec<Vec<i32>>, coefficients: &Vec<i32>) -> Vec<i32> {
    matrix
        .iter()
        .enumerate()
        .map(|(row_i, row)| {
            row.iter()
                .zip(coefficients.iter())
                .map(|(row_val, coeffval)| row_val * coeffval)
                .sum()
        })
        .collect()
}

fn delta(coefficients: &Vec<i32>, target: &Vec<i32>) -> i32 {
    coefficients
        .iter()
        .zip(target.iter())
        .map(|(c, t)| (c - t).abs())
        .sum()
}

pub fn search_constraints(
    matrix: &Vec<Vec<i32>>,
    constraints: &Vec<Constraint>,
    target: &Vec<i32>,
) -> Vec<i32> {
    let vertices = all_vertices(constraints, None);
    for vertex in &vertices {
        if product(matrix, &vertex) == *target {
            println!("Found solution: {:?} to target: {:?}", vertex, target);
            return vertex.clone();
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_form() {
        let matrix = Matrix::from_data(vec![
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 3.0],
            vec![0.0, 1.0, 0.0, 0.0, 1.0, 5.0],
            vec![0.0, 0.0, 1.0, 1.0, 0.0, 4.0],
            vec![1.0, 1.0, 0.0, 1.0, 0.0, 7.0],
        ]);

        println!("Original matrix:");
        matrix.print();
        println!("Reduced matrix:");
        let reduced_matrix = matrix.gaussian_form();
        println!("Reduced matrix:");
        reduced_matrix.print();
        let constraints = reduced_matrix.get_constraints();
        for (c, constraint) in constraints.iter().enumerate() {
            println!(
                "Constraint {c}: {:.2} <= x{c} <= {:.2}",
                constraint.lower_bound, constraint.upper_bound
            );
        }
        assert_eq!(
            reduced_matrix.data,
            vec![
                vec![1.0, 1.0, 0.0, 1.0, 0.0, 7.0],
                vec![0.0, 1.0, 0.0, 0.0, 1.0, 5.0],
                vec![0.0, 0.0, 1.0, 1.0, 0.0, 4.0],
                vec![0.0, 0.0, 0.0, 0.0, 1.0, 3.0]
            ]
        );

        let idata = matrix
            .data
            .clone()
            .into_iter()
            .map(|row| {
                row[..row.len() - 1]
                    .into_iter()
                    .map(|x| x.round() as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let p = product(&idata, &vec![1, 1, 1, 1, 1]);
        assert_eq!(p, vec![1, 2, 2, 3]);

        let coefficients = search_constraints(&idata, &constraints, &vec![3, 5, 4, 7]);
        println!("Coefficients: {:?}", coefficients);
        println!("Product: {:?}", product(&idata, &coefficients));
        assert_eq!(product(&idata, &coefficients), vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_product() {
        let matrix = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let coefficients = vec![1, 2, 3];
        let result = product(&matrix, &coefficients);
        assert_eq!(result, vec![14, 10]);
    }

    #[test]
    fn test_all_vertices() {
        let constraints = vec![
            Constraint::new(0.0, 1.0),
            Constraint::new(0.0, 2.0),
            Constraint::new(0.0, 3.0),
        ];
        let vertices = all_vertices(&constraints, None);
        for vertex in &vertices {
            println!("{:?}", vertex);
        }
        assert_eq!(
            vertices,
            vec![
                vec![0, 0, 0],
                vec![1, 0, 0],
                vec![0, 2, 0],
                vec![0, 0, 3],
                vec![1, 2, 0],
                vec![1, 0, 3],
                vec![0, 2, 3],
                vec![1, 2, 3],
            ]
        );
    }
}
