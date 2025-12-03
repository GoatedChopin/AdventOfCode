use std::fs;
use std::collections::HashMap;

fn read_input(path: &str) -> Vec<Vec<u64>> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>())
        .collect()
}

fn best_pair(input: &Vec<u64>) -> u64 {
    let mut one = 0;
    let mut one_index = 0;
    let mut two = 0;

    for i in 0..input.len() - 1 {
      if input[i] > one {
        one = input[i];
        one_index = i;
      }
    }

    for i in one_index + 1..input.len() {
      if input[i] > two {
        two = input[i];
      }
    }
    (one * 10) + two
}

fn part_one(input: &Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;
    for line in input {
        sum += best_pair(&line);
    }
    sum
}

fn combinations(range: usize, length: usize) -> Vec<Vec<usize>> {
  let input = (0..range).collect::<Vec<usize>>();
  let mut combinations = Vec::new();
  let length = input.len();
  fn backtrack(start: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if current.len() == length {
      result.push(current.clone());
      return;
    }
    for i in start..length {
      current.push(i);
      backtrack(i + 1, current, result);
      current.pop();
    }
  }
  backtrack(0, &mut Vec::new(), &mut combinations);
  return combinations;
}

fn best_n(input: &Vec<u64>, n: usize) -> u64 {
  let mut added_indexes = HashMap::new();
  let length = input.len();

  if n > length {
    return 0;
  }

  for p in 0..n {
    let mut best = 0;
    let mut best_index = 0;
    for i in 0..(length - (n - p)) {
      if input[i] > best && !added_indexes.contains_key(&i) {
        best = input[i];
        best_index = i;
      }
    }
    added_indexes.insert(best_index, p);
  }
    
  let mut out = 0;
  let mut current_position = 12;
  for i in 0..length {
    if !added_indexes.contains_key(&i) {
      continue;
    }
    out += input[i] * (10_u64.pow(current_position));
    current_position -= 1;
  }
  println!("Added indexes: {:?}", added_indexes);
  println!("Out: {}", out);
  out
}

fn part_two(input: &Vec<Vec<u64>>, n: usize) -> u64 {
  let mut sum = 0;
  for line in input {
    sum += best_n(&line, n);
  }
  sum
}

fn test() {
  let input = read_input("test.txt");
  assert_eq!(part_one(&input), 357);
  assert_eq!(part_two(&input, 12), 3121910778619);
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input, 12));
}
