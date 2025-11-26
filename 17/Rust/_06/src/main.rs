use std::fs;
use std::collections::HashSet;

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>()[0].clone()
}

fn part_one(input: &mut Vec<usize>) -> usize {
  let mut seen = HashSet::new();
  let mut steps = 0;
  let length = input.len();
  while !seen.contains(input) {
    steps += 1;
    seen.insert(input.clone());
    let max = *input.iter().max().unwrap();
    let index = input.iter().position(|x| x == &max).unwrap();
    input[index] = 0;
    for i in 0..max {
      input[(index + i + 1) % length] += 1;
    }
  }
  steps
}

fn part_two(input: &mut Vec<usize>) -> usize {
  let mut seen = HashSet::new();
  let mut steps = 0;
  let length = input.len();
  let start = input.clone();
  while !seen.contains(&start) {
    steps += 1;
    if steps % 1_000_000 == 0 {
      println!("Step: {}", steps);
    }
    let max = *input.iter().max().unwrap();
    let index = input.iter().position(|x| x == &max).unwrap();
    input[index] = 0;
    for i in 0..max {
      input[(index + i + 1) % length] += 1;
    }
    seen.insert(input.clone());
  }
  steps
}

fn take_step(input: &mut Vec<usize>) {
  let max = *input.iter().max().unwrap();
  let index = input.iter().position(|x| x == &max).unwrap();
  input[index] = 0;
  let length = input.len();
  for i in 0..max {
    input[(index + i + 1) % length] += 1;
  }
}

fn tortoise_and_hare(input: &mut Vec<usize>) -> usize {
  let mut tortoise = input.clone();
  let mut hare = input.clone();
  take_step(&mut tortoise);
  take_step(&mut hare);
  take_step(&mut hare);
  while tortoise.ne(&hare) {
    take_step(&mut hare);
    take_step(&mut hare);
    take_step(&mut tortoise);
  }

  let mut hare_steps = 2;
  let mut tortoise_steps = 1;
  take_step(&mut hare);
  take_step(&mut hare);
  take_step(&mut tortoise);
  while tortoise.ne(&hare) {
    hare_steps += 2;
    tortoise_steps += 1;
    take_step(&mut hare);
    take_step(&mut hare);
    take_step(&mut tortoise);
  }
  println!("Tortoise:\t{:?}\nHare:\t\t{:?}", tortoise, hare);
  hare_steps - tortoise_steps
}

fn main() {
    let mut input = read_input("input.txt");
    println!("Part one: {}", part_one(&mut input));
    let mut input = read_input("input.txt");
    println!("Part two: {}", tortoise_and_hare(&mut input));
}
