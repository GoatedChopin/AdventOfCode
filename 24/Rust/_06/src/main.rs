use std::{collections::{HashMap, HashSet}, fs};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
  row: i32,
  col: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    coord: Coordinate,
    dir: Direction,
}

fn step(position: Position) -> Position {
    match position.dir {
        Direction::Up => Position { coord: Coordinate { row: position.coord.row - 1, col: position.coord.col }, dir: position.dir },
        Direction::Down => Position { coord: Coordinate { row: position.coord.row + 1, col: position.coord.col }, dir: position.dir },
        Direction::Left => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col - 1 }, dir: position.dir },
        Direction::Right => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col + 1 }, dir: position.dir },
    }
}

fn turn(position: Position) -> Position {
    match position.dir {
        Direction::Up => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col }, dir: Direction::Right },
        Direction::Down => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col }, dir: Direction::Left },
        Direction::Left => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col }, dir: Direction::Up },
        Direction::Right => Position { coord: Coordinate { row: position.coord.row, col: position.coord.col }, dir: Direction::Down },
    }
}

struct Puzzle {
  walls: HashSet<Coordinate>,
  start: Position,
  rows: i32,
  cols: i32,
}

impl std::fmt::Display for Puzzle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..self.rows {
      for col in 0..self.cols {
        if self.walls.contains(&Coordinate { row, col }) {
          let _ = write!(f, "#");
        } else {
          let _ = write!(f, ".");
        }
      }
      let _ = write!(f, "\n");
    }
    Ok(())
  }
}

fn read_input(path: &str) -> Puzzle {
    let mut walls = HashSet::new();
    let lines: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut start = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
              walls.insert(Coordinate { row: row as i32, col: col as i32 });
            }
            if c == '^' {
                start = Some(Position { coord: Coordinate { row: row as i32, col: col as i32 }, dir: Direction::Up });
            }
        }
    }
    if start.is_none() {
      panic!("No start position found");
    }
    Puzzle { walls, start: start.unwrap(), rows: rows as i32, cols: cols as i32 }
}

fn in_bounds(coord: &Coordinate, puzzle: &Puzzle) -> bool {
  coord.row >= 0 && coord.row < puzzle.rows && coord.col >= 0 && coord.col < puzzle.cols
}

fn part_one(puzzle: &Puzzle) -> i32 {
  let mut visited = HashSet::new();
  let mut position = puzzle.start;
  while in_bounds(&position.coord, &puzzle) {
    visited.insert(position.coord.clone());
    let next = step(position);
    if puzzle.walls.contains(&next.coord) {
      position = turn(position);
    } else {
      position = next;
    }
  }
  visited.len() as i32
}

fn dfs(puzzle: &Puzzle, position: Position, memo: &mut HashMap<Position, bool>, possible_cycles: &mut HashSet<Position>, visited: &HashSet<Position>) -> bool {
  if memo.contains_key(&position) {
    return *memo.get(&position).unwrap();
  }
  let mut result = false;
  while in_bounds(&position.coord, &puzzle) {
    if memo.contains_key(&position) {
      return *memo.get(&position).unwrap();
    }
    if visited.contains(&position) {
      memo.insert(position.clone(), true);
      return true;
    }
    let next = step(position);
    if puzzle.walls.contains(&next.coord) {
      result = dfs(puzzle, turn(position), memo, possible_cycles, visited);
    } else {
      result = dfs(puzzle, next, memo, possible_cycles, visited);
    }
  }
  memo.insert(position.clone(), result);
  result
}

fn hits_cycle(puzzle: &Puzzle, position: Position, artificial_walls: &HashSet<Coordinate>) -> bool {
  let mut visited = HashSet::new();
  let mut position = position;
  while in_bounds(&position.coord, &puzzle) {
    if visited.contains(&position) {
      return true;
    }
    visited.insert(position.clone());
    let next = step(position);
    if puzzle.walls.contains(&next.coord) || artificial_walls.contains(&next.coord) {
      position = turn(position);
    } else {
      position = next;
    }
  }
  false
}

fn part_two(puzzle: &Puzzle) -> i32 {
  let mut possible_cycles: HashSet<Coordinate> = HashSet::new();
  let mut visited = HashSet::new();
  let mut position = puzzle.start;
  while in_bounds(&position.coord, &puzzle) {
    visited.insert(position.clone());
    let next = step(position);
    if puzzle.walls.contains(&next.coord) {
      position = turn(position);
    } else {
      position = next;
    }
  }
  for position in &visited {
    let artificial_wall = step(position.clone()).coord;
    if puzzle.walls.contains(&artificial_wall) {
      continue;
    }
    if hits_cycle(&puzzle, puzzle.start, &HashSet::from([artificial_wall])) {
      possible_cycles.insert(artificial_wall.clone());
    }
  }
  possible_cycles.len() as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_part_one() {
    let puzzle = read_input("test.txt");
    assert_eq!(part_one(&puzzle), 41);
  }

  #[test]
  fn test_part_two() {
    let puzzle = read_input("test.txt");
    assert_eq!(part_two(&puzzle), 6);
  }

  #[test]
  fn test_hits_cycle() {
    let puzzle = read_input("test.txt");
    let position = Position { coord: Coordinate { row: 8, col: 4 }, dir: Direction::Up };
    assert!(hits_cycle(&puzzle, position, &HashSet::from([Coordinate { row: 8, col: 3 }])));
    assert!(hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 9, col: 7 }])));
    assert!(hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 8, col: 1 }])));
    assert!(hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 7, col: 7 }])));
    assert!(hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 7, col: 6 }])));
    assert!(hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 6, col: 3 }])));
    assert!(!hits_cycle(&puzzle, puzzle.start, &HashSet::from([Coordinate { row: 6, col: 2 }])));
  }
}

fn main() {
    let puzzle = read_input("input.txt");
    println!("Part one: {}", part_one(&puzzle));
    println!("Part two: {}", part_two(&puzzle));
}
