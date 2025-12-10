use std::fs;

#[derive(Debug, Clone)]
struct Machine {
  desired: u8,
  buttons: Vec<u8>,
  current: u8,
}

impl Machine {
  fn new(desired: u8, buttons: Vec<u8>) -> Self {
    Self { desired, buttons, current: 0 }
  }

  fn from_str(s: &str) -> Self {
    let mut parts = s.split(" ");
    let desired_unformatted = parts.next().unwrap();
    let desired = desired_unformatted.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let buttons = parts.next().unwrap().split("").map(|x| x.parse().unwrap()).collect();
    Self { desired, buttons, current: 0 }
  }
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read input file")
}

fn main() {
    let input = read_input("input.txt");
    println!("{}", input);
}
