use std::fs;
use std::collections::HashSet;

#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

struct Delta {
    row: usize,
    rowpos: bool,
    col: usize,
    colpos: bool,
}

const DELTAS: [Delta; 8] = [
    // Right and Left
    Delta {
        row: 1,
        rowpos: true,
        col: 0,
        colpos: false,
    },
    Delta {
        row: 1,
        rowpos: false,
        col: 0,
        colpos: false,
    },
    // Up and Down
    Delta {
        row: 0,
        rowpos: false,
        col: 1,
        colpos: true,
    },
    Delta {
        row: 0,
        rowpos: false,
        col: 1,
        colpos: false,
    },
    // Diagonals
    Delta {
        row: 1,
        rowpos: true,
        col: 1,
        colpos: true,
    },
    Delta {
        row: 1,
        rowpos: false,
        col: 1,
        colpos: true,
    },
    Delta {
        row: 1,
        rowpos: true,
        col: 1,
        colpos: false,
    },
    Delta {
        row: 1,
        rowpos: false,
        col: 1,
        colpos: false,
    },
];

fn read_input(path: &str) -> Vec<Vec<char>> {
    println!("Reading input from {}", path);
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn move_by(position: &Coordinate, delta: &Delta) -> Coordinate {
    let row = if delta.rowpos {
        position.row + delta.row
    } else {
        position.row - delta.row
    };
    let col = if delta.colpos {
        position.col + delta.col
    } else {
        position.col - delta.col
    };
    Coordinate { row, col }
}

fn in_bounds(position: &Coordinate, delta: &Delta, grid_dimensions: &Coordinate) -> bool {
    // Check if moving by row will put position out of bounds
    let mut valid = true;
    if delta.row > 0 && !delta.rowpos {
        valid = valid && position.row > 0;
    }
    if delta.col > 0 && !delta.colpos {
        valid = valid && position.col > 0;
    }
    if delta.row > 0 && delta.rowpos {
        valid = valid && position.row + delta.row < grid_dimensions.row;
    }
    if delta.col > 0 && delta.colpos {
        valid = valid && position.col + delta.col < grid_dimensions.col;
    }
    valid
}

fn num_adjacent(
    input: &Vec<Vec<char>>,
    position: &Coordinate,
    grid_dimensions: &Coordinate,
) -> usize {
    let mut count = 0;
    for delta in DELTAS {
        if !in_bounds(position, &delta, grid_dimensions) {
            // let hypothetical_position = move_by(position, &delta);
            // println!("({}, {}) -> ({}{}, {}{}) is out of bounds, would be ({}, {})", position.row, position.col, if delta.rowpos { "" } else { "-" }, delta.row, if delta.colpos { "" } else { "-" }, delta.col, hypothetical_position.row, hypothetical_position.col);
            continue;
        }
        let new_position = move_by(position, &delta);
        if input[new_position.row][new_position.col] == '@' {
            count += 1;
        }
    }
    count
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    let grid_dimensions = Coordinate {
        row: input.len(),
        col: input[0].len(),
    };
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] != '@' {
                continue;
            }
            let n = num_adjacent(input, &Coordinate { row, col }, &grid_dimensions);
            if n < 4 {
                sum += 1;
            }
        }
    }
    sum
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    let grid_dimensions = Coordinate {
        row: input.len(),
        col: input[0].len(),
    };
    let mut mut_input = input.clone();

    let mut sum = 0;
    let mut can_remove = true;
    while can_remove {
        let mut sweep = HashSet::new();
        for row in 0..grid_dimensions.row {
            for col in 0..grid_dimensions.col {
                if mut_input[row][col] != '@' {
                    continue;
                }
                let n = num_adjacent(&mut_input, &Coordinate { row, col }, &grid_dimensions);
                if n < 4 {
                    sweep.insert(Coordinate { row, col });
                }
            }
        }
        if sweep.is_empty() {
            can_remove = false;
        }
        for coord in sweep {
            mut_input[coord.row][coord.col] = '.';
            sum += 1;
        }
    }
    sum
}

fn test() {
    let input = read_input("test.txt");
    let grid_dimensions = Coordinate {
        row: input.len(),
        col: input[0].len(),
    };
    assert_eq!(
        num_adjacent(&input, &Coordinate { row: 1, col: 9 }, &grid_dimensions),
        4
    );
    assert_eq!(part_one(&input), 13);
    assert_eq!(part_two(&input), 43);
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
