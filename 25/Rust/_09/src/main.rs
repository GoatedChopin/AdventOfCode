use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

fn read_input(file_path: &str) -> Vec<Coordinate> {
    let coordinates = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Coordinate {
                row: parts[0],
                col: parts[1],
            }
        })
        .collect::<Vec<Coordinate>>();

    coordinates
}

fn calculate_area(left: Coordinate, right: Coordinate) -> usize {
    let right_distance = if right.col > left.col {
        (right.col - left.col) + 1
    } else {
        (left.col - right.col) + 1
    };
    let left_distance = if left.row > right.row {
        (left.row - right.row) + 1
    } else {
        (right.row - left.row) + 1
    };
    right_distance * left_distance
}

fn part_one(coordinates: &Vec<Coordinate>) -> usize {
    let mut max_area = 0;
    for left_index in 0..coordinates.len() {
        let mut right_index = coordinates.len() - 1;
        while left_index < right_index {
            let left = coordinates[left_index];
            let right = coordinates[right_index];
            let area = calculate_area(left, right);
            // println!("left: {:?}, right: {:?}, area: {}", left, right, area);
            if area > max_area {
                // println!("New best: {:?} & {:?} -> {}", left, right, area);
                max_area = area;
            }
            right_index -= 1;
        }
    }
    max_area
}

fn zero_coordinate(coordinate: Coordinate, min_row: usize, min_col: usize) -> Coordinate {
    Coordinate {
        row: (coordinate.row - min_row) + 1,
        col: (coordinate.col - min_col) + 1,
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    print!(" ");
    for row in 0..grid.len() {
        print!("{}", row);
    }
    println!();
    for col in 0..grid[0].len() {
        print!("{}", col);
        for row in 0..grid.len() {
            print!("{}", grid[row][col]);
        }
        println!();
    }
    println!();
}

fn detect_enclosed_areas(grid: &Vec<Vec<char>>) -> HashSet<Coordinate> {
    let mut eligible_areas = HashSet::new();
    let mut ineligible_areas = HashSet::new();
    // Check left-to-right
    for row in 0..grid.len() {
        let mut num_walls_passed = 0;
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                num_walls_passed += 1;
                continue;
            }
            if num_walls_passed % 2 != 0 {
                eligible_areas.insert(Coordinate { row, col });
            } else if num_walls_passed == 0 {
                ineligible_areas.insert(Coordinate { row, col });
            }
        }
    }
    // Check top-to-bottom
    for col in 0..grid[0].len() {
      let mut num_walls_passed = 0;
      for row in 0..grid.len() {
        if grid[row][col] == '#' {
          num_walls_passed += 1;
          continue;
        }
        if num_walls_passed % 2 != 0 {
          eligible_areas.insert(Coordinate { row, col });
        } else if num_walls_passed == 0 {
          ineligible_areas.insert(Coordinate { row, col });
        }
      }
    }

    // Check right-to-left
    for row in (0..grid.len()).rev() {
      let mut num_walls_passed = 0;
      for col in 0..grid[row].len() {
        if grid[row][col] == '#' {
          num_walls_passed += 1;
          continue;
        }
        if num_walls_passed % 2 != 0 {
          eligible_areas.insert(Coordinate { row, col });
        } else if num_walls_passed == 0 {
          ineligible_areas.insert(Coordinate { row, col });
        }
      }
    }
    // Check bottom-to-top
    for col in (0..grid[0].len()).rev() {
      let mut num_walls_passed = 0;
      for row in 0..grid.len() {
        if grid[row][col] == '#' {
          num_walls_passed += 1;
          continue;
        }
        if num_walls_passed % 2 != 0 {
          eligible_areas.insert(Coordinate { row, col });
        } else if num_walls_passed == 0 {
          ineligible_areas.insert(Coordinate { row, col });
        }
      }
    }
    eligible_areas.difference(&ineligible_areas).cloned().collect()
}

fn build_grid(coordinates: &Vec<Coordinate>, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; cols]; rows];
    // println!("{:?}", coordinates);
    for i in 0..coordinates.len() - 1 {
        let current = coordinates[i];
        let next = coordinates[i + 1];
        let start_row = if current.row < next.row {
            current.row
        } else {
            next.row
        };
        let end_row = if current.row > next.row {
            current.row
        } else {
            next.row
        };
        let start_col = if current.col < next.col {
            current.col
        } else {
            next.col
        };
        let end_col = if current.col > next.col {
            current.col
        } else {
            next.col
        };
        // println!(
        //     "start_row: {}, start_col: {}, end_row: {}, end_col: {}",
        //     start_row, start_col, end_row, end_col
        // );
        for col in start_col..=end_col {
            grid[start_row][col] = '#';
        }
        for row in start_row..=end_row {
            grid[row][start_col] = '#';
        }
        // print_grid(&grid);
    }

    // Link index 0 to index -1
    let first = coordinates[0];
    let last = coordinates[coordinates.len() - 1];
    for col in first.col..=last.col {
        grid[first.row][col] = '#';
    }
    for row in first.row..=last.row {
        grid[row][first.col] = '#';
    }
    let start_row = if first.row < last.row {
        first.row
    } else {
        last.row
    };
    let end_row = if first.row > last.row {
        first.row
    } else {
        last.row
    };
    let start_col = if first.col < last.col {
        first.col
    } else {
        last.col
    };
    let end_col = if first.col > last.col {
        first.col
    } else {
        last.col
    };
    // println!(
    //     "first_row: {}, start_col: {}, last_row: {}, last_col: {}",
    //     start_row, start_col, end_row, end_col
    // );
    for col in start_col..=end_col {
        grid[start_row][col] = '#';
    }
    for row in start_row..=end_row {
        grid[row][start_col] = '#';
    }

    // Detect enclosed areas and fill them with '#'
    let enclosed_areas = detect_enclosed_areas(&grid);

    for enclosed_area in enclosed_areas {
        grid[enclosed_area.row][enclosed_area.col] = '#';
    }
    grid
}

fn is_valid_area(left: Coordinate, right: Coordinate, grid: &Vec<Vec<char>>) -> bool {
    let start_row = if left.row < right.row {
        left.row
    } else {
        right.row
    };
    let end_row = if left.row > right.row {
        left.row
    } else {
        right.row
    };
    let start_col = if left.col < right.col {
        left.col
    } else {
        right.col
    };
    let end_col = if left.col > right.col {
        left.col
    } else {
        right.col
    };
    for row in start_row..=end_row {
        for col in start_col..=end_col {
            if grid[row][col] != '#' {
                return false;
            }
        }
    }
    true
}

fn part_two(coordinates: &Vec<Coordinate>) -> usize {
    let min_row = coordinates.iter().map(|c| c.row).min().unwrap();
    let max_row = coordinates.iter().map(|c| c.row).max().unwrap();
    let min_col = coordinates.iter().map(|c| c.col).min().unwrap();
    let max_col = coordinates.iter().map(|c| c.col).max().unwrap();
    let zero_coordinates = coordinates
        .iter()
        .map(|c| zero_coordinate(*c, min_row, min_col))
        .collect::<Vec<Coordinate>>();

    // println!("coordinates: {:?}", coordinates);
    // println!("zero_coordinates: {:?}", zero_coordinates);

    let grid = build_grid(
        &zero_coordinates,
        max_row - min_row + 2,
        max_col - min_col + 2,
    );
    // print_grid(&grid);
    let mut max_area = 0;
    println!("{}", "#".repeat(if ((coordinates.len() / 2) + 1) > 100 { 100 } else { (coordinates.len() / 2) + 1 }));
    let print_every_x = ((coordinates.len() / 2) + 1) / if ((coordinates.len() / 2) + 1) > 100 { 100 } else { (coordinates.len() / 2) + 1 };
    for left_index in 0..((coordinates.len() / 2) + 1) {
        if left_index % print_every_x == 0 {
            print!(".");
        }
        let mut right_index = coordinates.len() - 1;
        while left_index < right_index {
            let left = zero_coordinates[left_index];
            let right = zero_coordinates[right_index];
            let area = calculate_area(left, right);
            if area <= max_area {
                right_index -= 1;
                continue;
            }
            let is_valid = is_valid_area(left, right, &grid);
            if !is_valid {
                right_index -= 1;
                continue;
            }
            // println!("valid: {:?} & {:?} -> {}", left, right, area);
            max_area = area;
            right_index -= 1;
        }
    }
    println!();
    max_area
}

fn test() {
    let coordinates = read_input("test.txt");
    assert_eq!(part_one(&coordinates), 50);
    assert_eq!(part_two(&coordinates), 24);
}

fn main() {
    test();
    let coordinates = read_input("input.txt");
    println!("{}", part_one(&coordinates));
    println!("{}", part_two(&coordinates));
}
