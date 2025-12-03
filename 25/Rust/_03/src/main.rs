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

fn best_n(input: &Vec<u64>, n: usize) -> u64 {
  let combinations = combinations(input.len(), n);

  let mut out = 0;
  for combination in combinations {
    let mut current_value = 0;
    let mut current_position = 12;
    for i in 0..n {
      current_value += input[combination[i]] * (10_u64.pow(current_position));
      current_position -= 1;
    }
    if current_value > out {
      out = current_value;
    }
  }
  out / 10
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
