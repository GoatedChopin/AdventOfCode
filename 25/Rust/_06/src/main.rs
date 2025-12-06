use std::fs;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<usize>,
    operator: Operation,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum InputState {
    Numbers,
    Operators,
}

fn read_input_one(path: &str) -> Vec<MathProblem> {
    let mut state = InputState::Numbers;
    let mut problems = Vec::new();
    let mut numbers = HashMap::new(); // store each number at (row, column)
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.trim().lines().collect::<Vec<&str>>();

    for (row, line) in lines.iter().enumerate() {
        for (col, seg) in line.split_whitespace().enumerate() {
            if *seg == *"*" || *seg == *"+" {
                state = InputState::Operators;
            }
            match state {
                InputState::Numbers => {
                    if seg.chars().all(|c| c.is_digit(10)) {
                        numbers.insert((row, col), seg.parse::<usize>().unwrap());
                    }
                }
                InputState::Operators => {
                    let operator: Operation;
                    if seg == "+" {
                        operator = Operation::Add;
                    } else if seg == "*" {
                        operator = Operation::Multiply;
                    } else {
                        panic!("Invalid operator: {}", seg);
                    }
                    let mut current_numbers = Vec::new();
                    for i in 0..(lines.len() - 1) {
                        if numbers.contains_key(&(i, col)) {
                            current_numbers.push(*numbers.get(&(i, col)).unwrap());
                        }
                    }
                    problems.push(MathProblem {
                        numbers: current_numbers,
                        operator,
                    });
                }
            }
        }
    }
    problems
}

fn read_input_two(path: &str) -> Vec<MathProblem> {
    let mut problems = Vec::new();
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines().collect::<Vec<&str>>();

    let mut dimensions = (0, 0);
    let mut charmap = HashMap::new(); // store each character at (row, column)
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            // println!("Row: {}, Col: {}, Char: {}", row, col, c);
            charmap.insert((row, col), c);
            dimensions = (max(dimensions.0, row), max(dimensions.1, col));
        }
    }

    let mut state = InputState::Numbers;
    let mut operator = Operation::Add;
    let mut current_numbers = Vec::new();
    for inv_col in 0..dimensions.1+1 {
        let col = dimensions.1 - inv_col;
        // println!("Col: {}", col);
        let mut number_slice = Vec::new();
        for row in 0..(dimensions.0+1) {
            if !charmap.contains_key(&(row, col)) {
                // println!("  Skipping row: {}, col: {}", row, col);
                continue;
            }
            let c = charmap.get(&(row, col)).unwrap();
            // println!("  Char: {}", c);
            if *c == '*' {
                state = InputState::Operators;
                operator = Operation::Multiply.clone();
            } else if *c == '+' {
                state = InputState::Operators;
                operator = Operation::Add.clone();
            } else if c.is_digit(10) {
                number_slice.push(*c);
            }
        }
        if number_slice.len() == 0 {
          continue;
        }
        let current_number = number_slice.iter().collect::<String>().parse::<usize>().unwrap();
        current_numbers.push(current_number);
        // println!("Current numbers: {:?}", current_numbers);
        if state == InputState::Operators {
          state = InputState::Numbers;
          problems.push(MathProblem {
            numbers: current_numbers.clone(),
            operator: operator.clone(),
          });
          current_numbers = Vec::new();
        }
    }
    // println!("Problems: {:?}", problems);
    problems
}

fn part_one(problems: &Vec<MathProblem>) -> usize {
    let mut sum = 0;
    for problem in problems {
        match problem.operator {
            Operation::Add => {
                sum += problem.numbers.iter().sum::<usize>();
            }
            Operation::Multiply => {
                sum += problem.numbers.iter().product::<usize>();
            }
        }
    }
    sum
}

fn test() {
    let problems = read_input_one("test.txt");
    assert_eq!(part_one(&problems), 4277556);
    let problems = read_input_two("test.txt");
    assert_eq!(part_one(&problems), 3263827);
}

fn main() {
    test();
    let problems = read_input_one("input.txt");
    println!("Part one: {}", part_one(&problems));
    let problems = read_input_two("input.txt");
    println!("Part two: {}", part_one(&problems));
}
