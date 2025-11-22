use std::fs;
use std::collections::HashSet;

fn read_input(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn part_one(input: &Vec<Vec<String>>) -> usize {
    let mut sum = 0;
    for row in input {
      let mut valid = true;
      let mut visited = HashSet::new();
      for cell in row {
        if visited.contains(cell) {
          valid = false;
          break;
        }
        visited.insert(cell.clone());
      }
      if valid {
        sum += 1;
      }
    }
    sum
}

fn part_two(input: &Vec<Vec<String>>) -> usize {
  let mut sum = 0;
  for row in input {
    let mut valid = true;
    let mut visited = HashSet::new();
    for cell in row {
      let mut chars: Vec<char> = cell.chars().collect();
      chars.sort();
      let sorted: String = chars.into_iter().collect();
      if visited.contains(&sorted) {
        valid = false;
        break;
      }
      visited.insert(sorted);
    }
    if valid {
      sum += 1;
    }
  }
  sum
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
