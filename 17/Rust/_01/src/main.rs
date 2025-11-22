use std::fs;

fn part_one(input: &Vec<char>) -> usize {
  let mut sum = 0;
  for i in 0..input.len() {
    let index = i;
    let next_index = (index + 1) % input.len();
    let current_char = input[index];
    let next_char = input[next_index];
    if current_char == next_char {
        sum += current_char.to_digit(10).unwrap();
    }
  }
  sum as usize
}

fn part_two(input: &Vec<char>) -> usize {
  let half_len = input.len() / 2;
  let mut sum = 0;
  for i in 0..input.len() {
    let index = i;
    let next_index = (index + half_len) % input.len();
    let current_char = input[index];
    let next_char = input[next_index];
    if current_char == next_char {
        sum += current_char.to_digit(10).unwrap();
    }
  }
  sum as usize
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string().chars().collect::<Vec<char>>();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
