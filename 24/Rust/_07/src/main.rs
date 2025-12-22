use std::fs;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Concat,
}

struct Equation {
    lhs: i64,
    rhs_strs: Vec<String>,
    rhs_ints: Vec<i64>,
}

fn read_input(path: &str) -> Vec<Equation> {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let lhs = parts[0].parse::<i64>().unwrap();
            let rhs_strs = parts[1]
                .split(" ")
                .map(|s| s.into())
                .collect::<Vec<String>>();
            let rhs_ints = rhs_strs.iter().map(|s| s.parse::<i64>().unwrap()).collect();
            Equation {
                lhs,
                rhs_strs,
                rhs_ints,
            }
        })
        .collect()
}

fn can_solve_recursive(
    equation: &Equation,
    allowed_operations: &Vec<Operation>,
    current_value: i64,
    position: usize,
) -> bool {
    if position >= equation.rhs_ints.len() {
        return current_value == equation.lhs;
    }

    if current_value > equation.lhs {
        return false;
    }

    for operation in allowed_operations {
        let next_value = match operation {
            Operation::Add => current_value + equation.rhs_ints[position],
            Operation::Mul => current_value * equation.rhs_ints[position],
            Operation::Concat => {
                format!("{}{}", current_value, equation.rhs_strs[position])
                    .parse::<i64>()
                    .unwrap()
            }
        };

        if can_solve_recursive(equation, allowed_operations, next_value, position + 1) {
            return true;
        }
    }

    false
}

fn can_solve(equation: &Equation, allowed_operations: &Vec<Operation>) -> bool {
    if equation.rhs_ints.is_empty() {
        return false;
    }
    can_solve_recursive(equation, allowed_operations, equation.rhs_ints[0], 1)
}

fn part_one(equations: &Vec<Equation>) -> i64 {
    let mut result = 0;
    for equation in equations {
        let can_solve = can_solve(equation, &vec![Operation::Add, Operation::Mul]);
        if can_solve {
            result += equation.lhs;
        }
    }
    result
}

fn part_two(equations: &Vec<Equation>) -> i64 {
  let mut result = 0;
  for equation in equations {
      let can_solve = can_solve(equation, &vec![Operation::Add, Operation::Mul, Operation::Concat]);
      if can_solve {
          result += equation.lhs;
      }
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let equations = read_input("test.txt");
        let result = part_one(&equations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_two() {
        let equations = read_input("test.txt");
        let result = part_two(&equations);
        assert_eq!(result, 11387);
    }
}

fn main() {
    let equations = read_input("input.txt");
    let result = part_one(&equations);
    println!("Part 1: {}", result);
    let result = part_two(&equations);
    println!("Part 2: {}", result);
}
