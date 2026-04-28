use std::fs;

#[derive(Clone, Copy)]
struct V2 {
  x: i32,
  y: i32,
}

impl V2 {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  fn add(&self, other: &V2) -> V2 {
    V2 {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

struct Light {
  position: V2,
  velocity: V2,
}

impl Light {
  fn new(position: V2, velocity: V2) -> Self {
    Self { position, velocity }
  }
  
  
  fn step(&self) -> Self {
    Self {
      position: self.position.add(&self.velocity),
      velocity: self.velocity,
    }
  }
}

struct Grid {
  lights: Vec<Light>,
}

impl Grid {
  fn new(lights: Vec<Light>) -> Self {
    Self { lights }
  }
  
  fn step(&self) -> Self {
    Self {
      lights: self.lights.iter().map(|l| l.step()).collect(),
    }
  }

  fn print(&self) {
    let min_x = self.lights.iter().map(|l| l.position.x).min().unwrap();
    let max_x = self.lights.iter().map(|l| l.position.x).max().unwrap();
    let min_y = self.lights.iter().map(|l| l.position.y).min().unwrap();
    let max_y = self.lights.iter().map(|l| l.position.y).max().unwrap();
    for y in min_y..=max_y {
      for x in min_x..=max_x {
        if self.lights.iter().any(|l| l.position.x == x && l.position.y == y) {
          print!("#");
        } else {
          print!(".");
        }
      }
      println!();
    }
  }
}

fn read_input(filename: &str) -> Vec<String> {
    let input = fs::read_to_string(filename).unwrap();
    input.lines().map(|line| line.to_string()).collect()
}

fn part_one(input: &str) -> String {


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_input() {
    let input = read_input("test.txt");
    assert_eq!(input.len(), 31);
  }
}