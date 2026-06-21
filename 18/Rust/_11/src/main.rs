use std::fs;
use std::io::Write;

use rayon::prelude::*;

fn read_input(path: &str) -> isize {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .parse()
        .expect("Failed to parse input")
}

struct FuelGrid {
    grid: Vec<Vec<isize>>,
}

impl FuelGrid {
    fn new(max_x: isize, max_y: isize, serial: isize) -> Self {
        let mut grid = Vec::new();
        for y in 1..=max_y {
            let mut row = Vec::new();
            for x in 1..=max_x {
                row.push(power_level(x, y, serial));
            }
            grid.push(row);
        }
        Self { grid }
    }

    fn get_square(&self, x: usize, y: usize, size: usize) -> isize {
        (0..size)
            .flat_map(|dy| (0..size).map(move |dx| self.get(x + dx, y + dy)))
            .sum()
    }

    fn get(&self, x: usize, y: usize) -> isize {
        if x >= self.grid[0].len() || y >= self.grid.len() {
            return 0;
        }
        self.grid[y][x]
    }
}

fn power_level(x: isize, y: isize, serial: isize) -> isize {
    ((x + 10) * y + serial) * (x + 10) % 1000 / 100 - 5
}

fn part_one(input: isize, square_size: usize, grid: &FuelGrid) -> (usize, usize, isize) {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for y in 0..(grid.grid.len() - 2) {
        for x in 0..(grid.grid[y].len() - 2) {
            let power = grid.get_square(x, y, square_size);
            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }
    print!("o");
    std::io::stdout().flush().unwrap_or_default();
    (max_x + 1, max_y + 1, max_power)
}

fn part_two(input: isize) -> (usize, usize, isize) {
  let grid = FuelGrid::new(300, 300, input);
  let height = grid.grid.len();
  let width = grid.grid[0].len();

  let mut max_power = isize::MIN;
  let (mut max_x, mut max_y, mut max_size) = (0, 0, 0);

  for top in 0..height {
      for left in 0..width {
          // size 1
          let mut power = grid.get(left, top);
          if power > max_power {
              (max_power, max_x, max_y, max_size) = (power, left, top, 1);
          }

          let (mut right, mut bottom, mut size) = (left, top, 1usize);
          while right + 1 < width && bottom + 1 < height {
              let (nr, nb) = (right + 1, bottom + 1);
              for y in top..=bottom {
                  power += grid.get(nr, y); // new right column
              }
              for x in left..=right {
                  power += grid.get(x, nb); // new bottom row
              }
              power += grid.get(nr, nb); // new corner
              right = nr;
              bottom = nb;
              size += 1;

              if power > max_power {
                  (max_power, max_x, max_y, max_size) = (power, left, top, size);
              }
          }
      }
  }
  (max_x + 1, max_y + 1, max_size as isize)
}

fn main() {
    let input = read_input("input.txt");
    let grid = FuelGrid::new(300, 300, input);
    println!("{:?}", part_one(input, 3, &grid));
    println!("Starting part two");
    println!("{:?}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (mut x, mut y, _) = part_one(18, 3);
        assert_eq!(x, 33);
        assert_eq!(y, 45);
        // assert_eq!(power, 29);
        (x, y, _) = part_one(42, 3);
        assert_eq!(x, 21);
        assert_eq!(y, 61);
        // assert_eq!(power, 30);
        // assert_eq!(part_one(42, 3), (21, 61));
    }

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
