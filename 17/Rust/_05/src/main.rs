use std::fs;

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part_one(input: &mut Vec<i32>) -> usize {
    let mut steps = 0;
    let mut current = 0;
    while current < input.len() {
      let next;
        if input[current] < 0 {
          next = current - (input[current].abs() as usize);
        } else {
          next = current + (input[current].abs() as usize);
        }
        input[current] = input[current] + 1;
        current = next;
        steps += 1;
    }
    steps
}

fn part_two(input: &mut Vec<i32>) -> usize {
  let mut steps = 0;
  let mut current = 0;
  while current < input.len() {
    let next;
      if input[current] < 0 {
        next = current - (input[current].abs() as usize);
      } else {
        next = current + (input[current].abs() as usize);
      }
      if input[current] >= 3 {
        input[current] -= 1;
      } else {
        input[current] += 1;
      }
      current = next;
      steps += 1;
  }
  steps
}

fn main() {
    let mut input = read_input("input.txt");
    println!("Part one: {}", part_one(&mut input));
    let mut input = read_input("input.txt");
    println!("Part two: {}", part_two(&mut input));
}
