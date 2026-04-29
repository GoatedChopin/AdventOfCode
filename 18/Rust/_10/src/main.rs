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

fn read_input(filename: &str) -> Grid {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.lines().map(|line| line.to_string()).collect::<Vec<String>>();
    let lights = lines.iter().map(|line| {
      let filtered = line.replace("position=<", "").replace("> velocity=<", "").replace(">", "").replace(",", "");
      let parts = filtered.split_whitespace().collect::<Vec<&str>>();
      let position_x = parts[0].parse().unwrap();
      let position_y = parts[1].parse().unwrap();
      let velocity_x = parts[2].parse().unwrap();
      let velocity_y = parts[3].parse().unwrap();
      Light::new(V2::new(position_x, position_y), V2::new(velocity_x, velocity_y))
    }).collect();
    Grid::new(lights)
}

fn part_one(grid: Grid) -> usize {
  let mut grid = grid;
  let mut best_step = 0;
  for i in 0..100000 {
    grid = grid.step();
    grid.print();
  }
  grid.print();
  "".to_string()
}

fn main() {
    let grid = read_input("input.txt");
    let result = part_one(grid);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_input() {
    let input = read_input("test.txt");
    assert_eq!(part_one(input), 31);
  }
}