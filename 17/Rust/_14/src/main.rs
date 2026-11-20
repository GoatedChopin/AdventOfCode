pub mod hash;

use std::collections::HashSet;
use std::fs;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
}

fn generate_hex(input: &str) -> String {
    let mut out = String::new();
    for hex in input.chars() {
        let value = hex.to_digit(16).unwrap();
        out += &format!("{:04b}", value)
    }
    out
}

fn generate_grid(input: &str) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for i in 0..128 {
        let mut row = Vec::new();
        let s: String = i.to_string();
        let current = input.to_string() + "-" + &s;
        let hash = hash::hash(&current.bytes().collect());
        for hex in hash.chars() {
            let value = hex.to_digit(16).unwrap();
            for bit in format!("{:04b}", value).chars() {
                row.push(if bit == '1' { 1 } else { 0 });
            }
        }
        out.push(row);
    }
    out
}

fn in_bounds(coord: (usize, usize), grid: &Vec<Vec<usize>>) -> bool {
    coord.0 < grid.len() && coord.1 < grid.get(0).unwrap_or(&Vec::new()).len()
}

fn neighbors(coord: (usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    use Direction::*;
    Direction::ALL
        .iter()
        .filter_map(|dir| match dir {
            North => {
                if coord.0 == 0 {
                    return None;
                }
                Some((coord.0.saturating_sub(1), coord.1))
            }
            South => Some((coord.0 + 1, coord.1)),
            East => Some((coord.0, coord.1 + 1)),
            West => {
                if coord.1 == 0 {
                    return None;
                }
                Some((coord.0, coord.1.saturating_sub(1)))
            }
        })
        .filter(|coord| in_bounds(*coord, grid))
        .collect()
}

fn walk_group(coord: (usize, usize), grid: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = Vec::from([coord]);
    while !queue.is_empty() {
        let Some(current) = queue.pop() else {
            continue;
        };
        for (row, col) in neighbors(current, grid) {
            if grid[row][col] == 0 {
                continue;
            }
            if visited.contains(&(row, col)) {
                continue;
            }
            queue.push((row, col));
            visited.insert((row, col));
        }
    }
    visited
}

fn part_one(grid: &Vec<Vec<usize>>) -> usize {
    grid.iter()
        .map(|row| row.iter().map(|u| *u).sum::<usize>())
        .sum()
}

fn part_two(grid: &Vec<Vec<usize>>) -> usize {
    let mut visited = HashSet::new();
    let mut groups = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 0 {
                continue;
            }
            if visited.contains(&(row, col)) {
                continue;
            }
            let current_group = walk_group((row, col), grid);
            visited.extend(current_group);
            groups += 1;
        }
    }
    groups
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("No input file");
    let grid = generate_grid(input.trim());
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex() {
        let input = "a0c20170";
        let hex = generate_hex(input);
        println!("{}", hex);
        println!("{}", "10100000110000100000000101110000");
        assert!(hex.starts_with("10100000110000100000000101110000"));
    }

    #[test]
    fn test_grid() {
        let input = fs::read_to_string("test.txt").expect("Test file must exist");
        let grid = generate_grid(input.trim());
        assert_eq!(grid[0][0..8], [1, 1, 0, 1, 0, 1, 0, 0]);
    }
}
