pub mod hash;

use std::fs;

fn generate_grid(input: String) -> Vec<Vec<bool>> {
    let mut out = Vec::new();
    for i in 0..128 {
        let mut row = Vec::new();
        let s: String = i.to_string();
        let current = input.clone() + &s;
        let hash = hash::hash(&current.bytes().collect());
        for hex in hash.chars() {
            let value = hex.to_digit(16).unwrap();
            for bit in format!("{:04b}", value).chars() {
                row.push(bit == '1');
            }
        }
        out.push(row);
    }
    out
}

fn part_one(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter()
        .map(|row| row.iter().fold(0, |acc, b| if *b { acc + 1 } else { acc }))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("No input file");
    let grid = generate_grid(input);
    println!("Part one: {}", part_one(&grid));
}
