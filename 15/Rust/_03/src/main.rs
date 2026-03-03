use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read input")
}

fn part_one(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0i32, 0i32);
    visited.insert((x, y));

    for ch in input.chars() {
        match ch {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => continue, // ignore newlines/whitespace
        }
        visited.insert((x, y));
    }

    visited.len()
}

fn part_two(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let (mut sx, mut sy) = (0i32, 0i32); // Santa
    let (mut rx, mut ry) = (0i32, 0i32); // Robo-Santa
    visited.insert((0, 0));

    for (i, ch) in input.chars().enumerate() {
        let (x, y) = if i % 2 == 0 {
            (&mut sx, &mut sy)
        } else {
            (&mut rx, &mut ry)
        };
        match ch {
            '^' => *y += 1,
            'v' => *y -= 1,
            '>' => *x += 1,
            '<' => *x -= 1,
            _ => continue,
        }
        visited.insert((*x, *y));
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part_one() {
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn examples_part_two() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
