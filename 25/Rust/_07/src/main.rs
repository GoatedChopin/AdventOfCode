use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Dot,
    Beam,
    Start,
    Splitter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

fn read_input(path: &str) -> Vec<Vec<Cell>> {
    let chars = fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid = Vec::new();
    for line in chars {
        let mut row = Vec::new();
        for cell in line {
            row.push(match cell {
                '.' => Cell::Dot,
                '|' => Cell::Beam,
                'S' => Cell::Start,
                '^' => Cell::Splitter,
                _ => panic!("Invalid cell: {}", cell),
            });
        }
        grid.push(row);
    }
    grid
}

fn find_start(grid: &Vec<Vec<Cell>>) -> Position {
    // Doesn't this just give us the row, not the column?
    let row = grid
        .iter()
        .position(|row| row.iter().any(|cell| cell == &Cell::Start))
        .unwrap();
    let col = grid[row]
        .iter()
        .position(|cell| cell == &Cell::Start)
        .unwrap();
    Position {
        row: row,
        col: col,
        direction: Direction::Down,
    }
}

fn in_bounds(position: &Position, grid: &Vec<Vec<Cell>>) -> bool {
    position.row < grid.len() && position.col < grid[0].len()
}

fn part_one(grid: &Vec<Vec<Cell>>) -> usize {
    let start = find_start(grid);

    let mut queue = Vec::new();
    queue.push(start);

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut num_splits = 0;
    while !queue.is_empty() {
        let position = queue.pop().unwrap();
        if !in_bounds(&position, grid) {
            continue;
        }
        let cell = grid[position.row][position.col];
        match cell {
            Cell::Splitter => {
                if visited.contains(&position) {
                    continue;
                }
                visited.insert(position);
                num_splits += 1;
                let mut new_positions = Vec::new();
                let right = Position {
                    row: position.row,
                    col: position.col + 1,
                    direction: position.direction,
                };
                if position.col > 0 {
                    new_positions.push(Position {
                        row: position.row,
                        col: position.col - 1,
                        direction: position.direction,
                    });
                }
                new_positions.push(right);
                for new_position in new_positions.iter() {
                    if visited.contains(&new_position) {
                        continue;
                    }
                    visited.insert(*new_position);
                    if in_bounds(&new_position, grid) {
                        queue.push(*new_position);
                    }
                }
            }
            _ => {
              let new_position = Position { row: position.row + 1, col: position.col, direction: position.direction };
              if in_bounds(&new_position, grid) {
                queue.push(new_position);
              }
            }
        }
    }

    num_splits
}

fn part_two(grid: &Vec<Vec<Cell>>) -> usize {
  let start = find_start(grid);

  let mut queue = Vec::new();
  queue.push(start);

  let mut visited = HashSet::new();
  visited.insert(start);
  let mut num_paths = 0;
  while !queue.is_empty() {
      let position = queue.pop().unwrap();
      if !in_bounds(&position, grid) {
          continue;
      }
      let cell = grid[position.row][position.col];
      match cell {
          Cell::Splitter => {
              let right = Position {
                  row: position.row,
                  col: position.col + 1,
                  direction: position.direction,
              };
              let left = Position {
                  row: position.row,
                  col: position.col - 1,
                  direction: position.direction,
              };
              if position.col > 0 && in_bounds(&left, grid) {
                visited.insert(left);
                queue.push(left);
              } else {
                num_paths += 1;
              }
              if in_bounds(&right, grid) {
                visited.insert(right);
                queue.push(right);
              } else {
                num_paths += 1;
              }
          }
          _ => {
            let new_position = Position { row: position.row + 1, col: position.col, direction: position.direction };
            if in_bounds(&new_position, grid) {
              queue.push(new_position);
            } else {
              num_paths += 1;
            }
          }
      }
  }

  num_paths
}

fn test() {
    let grid = read_input("test.txt");
    assert_eq!(part_one(&grid), 21);
    assert_eq!(part_two(&grid), 40);
}

fn main() {
    test();
    let grid = read_input("input.txt");
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}
