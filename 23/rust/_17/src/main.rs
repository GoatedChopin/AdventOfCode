use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn move_by(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Point {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Point {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Point {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

struct City {
    coordinates: HashMap<Point, isize>,
    rows: isize,
    cols: isize,
}

impl City {
    fn in_bounds(&self, point: Point) -> bool {
        self.coordinates.contains_key(&point)
    }

    fn render(&self, visited: &HashSet<Point>) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let point = Point {
                    row: row as isize,
                    col: col as isize,
                };
                if visited.contains(&point) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn read_input(path: &str) -> City {
    let lines = fs::read_to_string(path).expect("Failed to read input file");
    let mut city = City {
        coordinates: HashMap::new(),
        rows: 0,
        cols: 0,
    };
    for (row, line) in lines.lines().enumerate() {
        city.rows += 1;
        city.cols = city.cols.max(line.len() as isize);
        for (col, char) in line.chars().enumerate() {
            if let Some(d) = char.to_digit(10) {
                city.coordinates.insert(
                    Point {
                        row: row as isize,
                        col: col as isize,
                    },
                    d as isize,
                );
            }
        }
    }
    city
}

struct SearchState {
    point: Point,
    direction: Direction,
    consecutive_steps_forward: isize,
    cost: isize,
    visited: HashSet<Point>,
}

impl SearchState {
    fn valid_directions(&self, city: &City) -> Vec<Direction> {
        let mut directions = Vec::new();
        if self.consecutive_steps_forward < 3 {
            if city.in_bounds(self.point.move_by(self.direction)) {
                directions.push(self.direction);
            }
        }
        let left_dir = self.direction.turn_left();
        let left_point = self.point.move_by(left_dir);
        if city.in_bounds(left_point) {
            directions.push(self.direction.turn_left());
        }
        let right_dir = self.direction.turn_right();
        let right_point = self.point.move_by(right_dir);
        if city.in_bounds(right_point) {
            directions.push(self.direction.turn_right());
        }
        directions
    }

    fn valid_directions2(&self, city: &City) -> Vec<Direction> {
        let mut directions = Vec::new();
        if self.consecutive_steps_forward < 10 {
            if city.in_bounds(self.point.move_by(self.direction)) {
                directions.push(self.direction);
            }
        }
        if self.consecutive_steps_forward < 4 {
            return directions;
        }
        let left_dir = self.direction.turn_left();
        let left_point = self.point.move_by(left_dir);
        if city.in_bounds(left_point) {
            directions.push(self.direction.turn_left());
        }
        let right_dir = self.direction.turn_right();
        let right_point = self.point.move_by(right_dir);
        if city.in_bounds(right_point) {
            directions.push(self.direction.turn_right());
        }
        directions
    }

    fn move_by(&self, direction: Direction, city: &City) -> SearchState {
        let new_point = self.point.move_by(direction);
        let new_cost = self.cost + city.coordinates[&new_point];
        let new_consecutive_steps_forward = if direction == self.direction {
            self.consecutive_steps_forward + 1
        } else {
            1
        };
        let mut new_visited = self.visited.clone();
        new_visited.insert(new_point.clone());
        SearchState {
            point: new_point,
            direction: direction,
            consecutive_steps_forward: new_consecutive_steps_forward,
            cost: new_cost,
            visited: new_visited,
        }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SearchState {}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn search(city: &City) -> isize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(SearchState {
        point: Point { row: 0, col: 0 },
        direction: Direction::Right,
        consecutive_steps_forward: 0,
        cost: 0,
        visited: HashSet::new(),
    });
    queue.push(SearchState {
        point: Point { row: 0, col: 0 },
        direction: Direction::Down,
        consecutive_steps_forward: 0,
        cost: 0,
        visited: HashSet::new(),
    });
    while let Some(state) = queue.pop() {
        if state.point.row == city.rows - 1 && state.point.col == city.cols - 1 {
            city.render(&state.visited);
            return state.cost;
        }

        for direction in state.valid_directions(city) {
            let new_state = state.move_by(direction, city);
            let previous_cost = visited
                .get(&(
                    new_state.point,
                    new_state.direction,
                    new_state.consecutive_steps_forward,
                ))
                .unwrap_or(&isize::MAX);
            if new_state.cost >= *previous_cost {
                continue;
            }
            visited.insert(
                (
                    new_state.point.clone(),
                    new_state.direction,
                    new_state.consecutive_steps_forward,
                ),
                new_state.cost,
            );
            queue.push(new_state);
        }
    }
    return 0;
}

fn search2(city: &City) -> isize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    queue.push(SearchState {
        point: Point { row: 0, col: 0 },
        direction: Direction::Right,
        consecutive_steps_forward: 0,
        cost: 0,
        visited: HashSet::new(),
    });
    queue.push(SearchState {
        point: Point { row: 0, col: 0 },
        direction: Direction::Down,
        consecutive_steps_forward: 0,
        cost: 0,
        visited: HashSet::new(),
    });
    while let Some(state) = queue.pop() {
        if state.point.row == city.rows - 1 && state.point.col == city.cols - 1 {
            city.render(&state.visited);
            return state.cost;
        }

        for direction in state.valid_directions2(city) {
            let new_state = state.move_by(direction, city);
            let previous_cost = visited
                .get(&(
                    new_state.point,
                    new_state.direction,
                    new_state.consecutive_steps_forward,
                ))
                .unwrap_or(&isize::MAX);
            if new_state.cost >= *previous_cost {
                continue;
            }
            visited.insert(
                (
                    new_state.point.clone(),
                    new_state.direction,
                    new_state.consecutive_steps_forward,
                ),
                new_state.cost,
            );
            queue.push(new_state);
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        let city = read_input("test.txt");
        assert_eq!(search(&city), 102);
    }

    #[test]
    fn test_search2() {
        let city = read_input("test.txt");
        assert_eq!(search2(&city), 94);
    }
}

fn main() {
    let city = read_input("input.txt");
    println!("{}", search(&city));
    println!("{}", search2(&city));
}
