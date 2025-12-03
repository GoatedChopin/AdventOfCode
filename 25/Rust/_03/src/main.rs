use std::fs;

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

fn combinations(range: usize, group_size: usize) -> Vec<Vec<usize>> {
  let input = (0..range).collect::<Vec<usize>>();
  let mut combinations = Vec::new();
  fn backtrack(start: usize, group_size: usize, current: &mut Vec<usize>, input: &Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if current.len() == group_size {
      result.push(current.clone());
      return;
    }
    for i in start..input.len() {
      current.push(input[i]);
      backtrack(i + 1, group_size, current, input, result);
      current.pop();
    }
  }
  backtrack(0, group_size, &mut Vec::new(), &input, &mut combinations);
  return combinations;
}

fn greater_than(left: Vec<usize>, right: Vec<usize>) -> bool {
  if left.len() != right.len() {
    return left.len() > right.len();
  }
  for (i, j) in left.iter().zip(right.iter()) {
    if i > j {
      return true;
    }
    if i < j {
      return false;
    }
  }
  false
}

fn compute_value(input: &Vec<usize>) -> usize {
  let mut current_value = 0;
  let mut current_position = input.len();
  for i in 0..input.len() {
    current_value += input[i] * (10_usize.pow(current_position));
    current_position -= 1;
  }
  current_value
}

fn best_n(input: &Vec<u64>, n: usize) -> u64 {
  
  let mut best = vec![0; n];
  for start in 0..(input.len() - n) {
    let mut current = Vec::new();
    for i in start..(start + n) {
      current.push(i);
    }
    for i in (start + n)..input.len() {
      if current[current.len() - 1] < input[i] {
        stack.pop();
        stack.push(input[i]);
      }
    }
    if greater_than(current, best) {
      best = current.clone();
    }
  }

  return compute_value(&best) as u64;
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
