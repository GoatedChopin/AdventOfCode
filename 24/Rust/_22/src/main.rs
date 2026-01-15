use std::fs;

fn mix(secret_number: usize, number: usize) -> usize {
  secret_number ^ number
}

fn prune(secret_number: usize) -> usize {
  secret_number % 16777216
}

#[derive(Debug, Clone)]
struct Buyer {
  secret_number: usize,
  deltas: Vec<usize>,
  delta_index: usize,
}

impl Buyer {
  fn new(number: usize) -> Self {
    Self { secret_number: number, deltas: vec![0; 4], delta_index: 0 }
  }

  fn next(&mut self) -> Self {
    // multiply by 64
    let new_number = self.secret_number * 64;

    // mix
    self.secret_number = mix(self.secret_number, new_number);

    // prune
    self.secret_number = prune(self.secret_number);

    // divide by 32
    let new_number = self.secret_number / 32;

    // mix
    self.secret_number = mix(self.secret_number, new_number);

    // prune
    self.secret_number = prune(self.secret_number);

    // multiply by 2048
    let new_number = self.secret_number * 2048;

    // mix
    self.secret_number = mix(self.secret_number, new_number);

    // prune
    self.secret_number = prune(self.secret_number);

    let mut new_deltas = self.deltas.clone();
    new_deltas[self.delta_index] = self.buy_price();

    Self { secret_number: self.secret_number, deltas: new_deltas, delta_index: (self.delta_index + 1) % 4 }
  }

  fn buy_price(&self) -> usize {
    self.secret_number % 10
  }

  fn buy_deltas(&self) -> Vec<usize> {
    let mut deltas = vec![0; 4];
    let mut current_index = 0;
    for i in self.delta_index..self.deltas.len() {
      deltas[current_index] = self.deltas[i];
      current_index += 1;
    }
    for i in 0..self.delta_index {
      deltas[current_index] = self.deltas[i];
      current_index += 1;
    }
    deltas
  }
}

fn read_input(file_path: &str) -> Vec<Buyer> {
    fs::read_to_string(file_path).expect("Failed to read file").lines().map(|line| Buyer::new(line.parse().unwrap())).collect()
}

fn part_one(input: Vec<Buyer>) -> usize {
    let mut buyers = input.clone();
    for buyer in buyers.iter_mut() {
        for _ in 0..2000 {
          *buyer = buyer.next();
        }
    }
    buyers.iter().map(|buyer| buyer.secret_number).sum()
}

fn part_two(input: Vec<Buyer>) -> usize {
    0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mix() {
    assert_eq!(mix(0, 0), 0);
  }

  #[test]
  fn test_prune() {
    assert_eq!(prune(100000000), 16113920);
  }

  #[test]
  fn test_part_one() {
    let input = read_input("test.txt");
    assert_eq!(part_one(input), 37327623);
  }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input));
}
