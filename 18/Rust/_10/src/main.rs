use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone)]
struct Grid {
    lights: Vec<Light>,
}

impl Into<String> for Grid {
    fn into(self) -> String {
        let mut s = String::new();
        let min_x = self.lights.iter().map(|l| l.position.x).min().unwrap();
        let max_x = self.lights.iter().map(|l| l.position.x).max().unwrap();
        let min_y = self.lights.iter().map(|l| l.position.y).min().unwrap();
        let max_y = self.lights.iter().map(|l| l.position.y).max().unwrap();

        let positions: HashSet<V2> = self.lights.iter().map(|l| l.position).collect();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if positions.contains(&V2::new(x, y)) {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }

        s
    }
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

    fn bbox_area(&self) -> i64 {
        let min_x = self.lights.iter().map(|l| l.position.x).min().unwrap();
        let max_x = self.lights.iter().map(|l| l.position.x).max().unwrap();
        let min_y = self.lights.iter().map(|l| l.position.y).min().unwrap();
        let max_y = self.lights.iter().map(|l| l.position.y).max().unwrap();
        (max_x - min_x) as i64 * (max_y - min_y) as i64
    }
}

fn read_input(filename: &str) -> Grid {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<String> = input
        .lines()
        .map(|line| {
            line.to_string()
                .replace("position=<", "")
                .replace(",", "")
                .replace("> velocity=<", " ")
                .replace(">", "")
        })
        .collect();

    let lights = lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            Light::new(
                V2 {
                    x: parts[0].parse().expect("Bad integer input"),
                    y: parts[1].parse().expect("Bad integer input"),
                },
                V2 {
                    x: parts[2].parse().expect("Bad integer input"),
                    y: parts[3].parse().expect("Bad integer input"),
                },
            )
        })
        .collect();

    Grid::new(lights)
}

fn part_one_and_two(input: Grid) -> (String, usize) {
  let mut grid = input;
  let mut prev_area = grid.bbox_area();
  for i in 1.. {
      let next = grid.step();
      let area = next.bbox_area();
      if area > prev_area {
          return (grid.into(), i - 1); // also gives you part 2
      }
      grid = next;
      prev_area = area;
  }
  unreachable!()
}

fn main() {
    let input = read_input("input.txt");
    let (message, steps) = part_one_and_two(input);
    println!("{}\n{}", message, steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("test.txt");
        let (_message, steps) = part_one_and_two(input);
        assert_eq!(steps, 3);
    }
}
