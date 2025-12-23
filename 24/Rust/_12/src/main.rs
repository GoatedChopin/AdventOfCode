use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Shape {
    char: char,
    area: i32,
    points: Vec<Point>,
    walls: Vec<Point>,
    perimeter: i32,
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn in_bounds(input: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && row < input.len() as i32 && col >= 0 && col < input[0].len() as i32
}

fn get_vector(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn num_neighbors(input: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {
    let mut neighbors = 0;
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (dr, dc) = get_vector(direction);
        let new_row = row + dr;
        let new_col = col + dc;
        if !in_bounds(input, new_row, new_col) {
            continue;
        }
        if input[new_row as usize][new_col as usize] == input[row as usize][col as usize] {
            neighbors += 1;
        }
    }
    neighbors
}

fn get_neighbors(input: &Vec<Vec<char>>, row: i32, col: i32) -> Vec<Point> {
    let mut neighbors = Vec::new();
    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (dr, dc) = get_vector(direction);
        let new_row = row + dr;
        let new_col = col + dc;
        if !in_bounds(input, new_row, new_col) {
            continue;
        }
        neighbors.push(Point {
            row: new_row,
            col: new_col,
        });
    }
    neighbors
}

fn walk(input: &Vec<Vec<char>>, row: usize, col: usize) -> Shape {
    let char = input[row][col];
    let mut points = Vec::new();
    let mut walls = Vec::new();
    let mut perimeter: i32 = 0;
    let current_point = Point {
        row: row as i32,
        col: col as i32,
    };
    points.push(current_point);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(current_point);
    visited.insert(current_point); // Mark starting point as visited immediately
    while let Some(current_point) = queue.pop_front() {
        let neighbors = get_neighbors(input, current_point.row, current_point.col);
        let chars = neighbors
            .iter()
            .map(|n| input[n.row as usize][n.col as usize])
            .collect::<Vec<char>>();
        let non_matching_chars = chars.iter().filter(|c| *c != &char).collect::<Vec<&char>>();
        if non_matching_chars.len() > 0 {
            walls.push(current_point);
        }
        let out_of_bounds_neighbors = 4 - neighbors.len() as i32;
        // Perimeter increases but only if the current point isn't surrounded by the same character as itself.
        perimeter += non_matching_chars.len() as i32 + out_of_bounds_neighbors;
        for neighbor in neighbors {
            if input[neighbor.row as usize][neighbor.col as usize] == char
                && !visited.contains(&neighbor)
            {
                points.push(neighbor.clone());
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    Shape {
        char,
        points,
        walls,
        perimeter,
        area: visited.len() as i32,
    }
}

fn part_one(input: &Vec<Vec<char>>) -> i32 {
    let mut total_cost = 0;
    let mut visited = HashSet::new();
    for (row, line) in input.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            let current_point = Point {
                row: row as i32,
                col: col as i32,
            };
            if !visited.contains(&current_point) {
                let shape = walk(input, row, col);
                println!(
                    "Shape: {} -> area: {}, perimeter: {}",
                    shape.char, shape.area, shape.perimeter
                );
                total_cost += shape.perimeter * shape.area;
                visited.extend(shape.points);
            }
        }
    }
    total_cost
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.row - p2.row).abs() + (p1.col - p2.col).abs()
}

fn get_next_point(point: &Point, direction: Direction) -> Point {
    let (dr, dc) = get_vector(direction);
    Point {
        row: point.row + dr,
        col: point.col + dc,
    }
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_carat(direction: Direction) -> char {
    match direction {
        Direction::Up => '^',
        Direction::Down => 'v',
        Direction::Left => '<',
        Direction::Right => '>',
    }
}

fn print_path(input: &Vec<Vec<char>>, path: &HashMap<Point, Direction>) {
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let point = Point { row: row as i32, col: col as i32 };
            if path.contains_key(&point) {
                print!("{}", get_carat(*path.get(&point).unwrap()));
            } else {
                print!("{}", input[row][col]);
            }
        }
        println!();
    }
}

fn num_sides(shape: &Shape) -> i32 {
    let point_set: HashSet<Point> = shape.points.iter().cloned().collect();
    let mut corners = 0;
    
    for point in &shape.points {
        let row = point.row;
        let col = point.col;
        
        // Check all 4 diagonal neighbors and their adjacent cells
        // For each diagonal, check if it forms a corner
        
        // Top-left corner cases
        let top = point_set.contains(&Point { row: row - 1, col });
        let left = point_set.contains(&Point { row, col: col - 1 });
        let top_left = point_set.contains(&Point { row: row - 1, col: col - 1 });
        
        // Outer corner: neither top nor left are in shape
        if !top && !left {
            corners += 1;
        }
        // Inner corner: both top and left are in shape, but diagonal is not
        if top && left && !top_left {
            corners += 1;
        }
        
        // Top-right corner cases
        let right = point_set.contains(&Point { row, col: col + 1 });
        let top_right = point_set.contains(&Point { row: row - 1, col: col + 1 });
        
        if !top && !right {
            corners += 1;
        }
        if top && right && !top_right {
            corners += 1;
        }
        
        // Bottom-left corner cases
        let bottom = point_set.contains(&Point { row: row + 1, col });
        let bottom_left = point_set.contains(&Point { row: row + 1, col: col - 1 });
        
        if !bottom && !left {
            corners += 1;
        }
        if bottom && left && !bottom_left {
            corners += 1;
        }
        
        // Bottom-right corner cases
        let bottom_right = point_set.contains(&Point { row: row + 1, col: col + 1 });
        
        if !bottom && !right {
            corners += 1;
        }
        if bottom && right && !bottom_right {
            corners += 1;
        }
    }
    
    corners
}

fn part_two(input: &Vec<Vec<char>>) -> i32 {
    let mut total_cost = 0;
    let mut visited = HashSet::new();
    for (row, line) in input.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            let current_point = Point {
                row: row as i32,
                col: col as i32,
            };
            if !visited.contains(&current_point) {
                let shape = walk(input, row, col);
                let sides = num_sides(&shape);
                println!(
                    "Shape: {} -> area: {}, sides: {}",
                    shape.char, shape.area, sides
                );
                total_cost += sides * shape.area;
                visited.extend(shape.points);
            }
        }
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(&input), 1930);
    }

    #[test]
    fn test_num_sides() {
        let input = read_input("test.txt");
        let shape = walk(&input, 0, 0);
        assert_eq!(num_sides(&shape), 10);
        let shape = walk(&input, 0, 4);
        assert_eq!(num_sides(&shape), 4);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("test.txt");
        assert_eq!(part_two(&input), 1206);
    }
}

fn main() {
    let input = read_input("input.txt");
    let result = part_one(&input);
    println!("Part one: {}", result);
    let result = part_two(&input);
    println!("Part two: {}", result);
}
