use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn move_by(&self, velocity: Point) -> Point {
        Point {
            row: self.row + velocity.row,
            col: self.col + velocity.col,
        }
    }

    fn wrap(&self, max_row: i32, max_col: i32) -> Point {
        Point {
            row: self.row.rem_euclid(max_row),
            col: self.col.rem_euclid(max_col),
        }
    }
}

#[derive(Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn from_str(s: &str) -> Robot {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let p = parts[0];
        let v = parts[1];
        let p_vec: Vec<i32> = p
            .replace("p=", "")
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let v_vec: Vec<i32> = v
            .replace("v=", "")
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let [p_row, p_col]: [i32; 2] = [p_vec[1], p_vec[0]];
        let [v_row, v_col]: [i32; 2] = [v_vec[1], v_vec[0]];
        Robot {
            position: Point {
                row: p_row,
                col: p_col,
            },
            velocity: Point {
                row: v_row,
                col: v_col,
            },
        }
    }

    fn step(&self, max_row: i32, max_col: i32) -> Robot {
        Robot {
            position: self.position.move_by(self.velocity).wrap(max_row, max_col),
            velocity: self.velocity,
        }
    }
}

fn read_input(file_path: &str) -> Vec<Robot> {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .map(|line| Robot::from_str(line))
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn get_quadrant(robot: Robot, max_row: i32, max_col: i32) -> Option<Quadrant> {
    // Exclude the center row and the center column
    let center_row = max_row / 2;
    let center_col = max_col / 2;
    if robot.position.row == center_row || robot.position.col == center_col {
        return None;
    }
    if robot.position.row < max_row / 2 && robot.position.col < max_col / 2 {
        Some(Quadrant::TopLeft)
    } else if robot.position.row < max_row / 2 && robot.position.col >= max_col / 2 {
        Some(Quadrant::TopRight)
    } else if robot.position.row >= max_row / 2 && robot.position.col < max_col / 2 {
        Some(Quadrant::BottomLeft)
    } else {
        Some(Quadrant::BottomRight)
    }
}

fn visualize(robots: &Vec<Robot>, max_row: i32, max_col: i32, split: &str) {
    let mut grid = vec![vec![0; max_col as usize]; max_row as usize];
    for robot in robots {
        grid[robot.position.row as usize][robot.position.col as usize] += 1;
    }
    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
            print!("{split}");
        }
        println!();
    }
}

fn part_one(robots: Vec<Robot>, max_row: i32, max_col: i32) -> i32 {
    let mut robots = robots;
    for _ in 0..100 {
        visualize(&robots, max_row, max_col, " ");
        robots = robots
            .iter()
            .map(|robot| robot.step(max_row, max_col))
            .collect();
    }
    visualize(&robots, max_row, max_col, " ");
    // Count the robots in each Quadrant, multiply the counts together
    let mut quadrant_counts = HashMap::new();
    for robot in robots {
        let q = get_quadrant(robot, max_row, max_col);
        if q.is_none() {
            continue;
        }
        let q = q.unwrap();
        *quadrant_counts.entry(q).or_insert(0) += 1;
    }
    let top_left_count = quadrant_counts.get(&Quadrant::TopLeft).unwrap_or(&0);
    let top_right_count = quadrant_counts.get(&Quadrant::TopRight).unwrap_or(&0);
    let bottom_left_count = quadrant_counts.get(&Quadrant::BottomLeft).unwrap_or(&0);
    let bottom_right_count = quadrant_counts.get(&Quadrant::BottomRight).unwrap_or(&0);
    // Print the quadrant counts
    println!("Top Left: {:?}", top_left_count);
    println!("Top Right: {:?}", top_right_count);
    println!("Bottom Left: {:?}", bottom_left_count);
    println!("Bottom Right: {:?}", bottom_right_count);
    top_left_count * top_right_count * bottom_left_count * bottom_right_count
}

// Interesting pictures will have many neighbors within 1 step of each other
fn interest(robots: &Vec<Robot>) -> i32 {
    let mut neighbors = 0;
    let mut distinct_positions = HashSet::new();
    for robot in robots {
        distinct_positions.insert(robot.position);
    }
    for robot in robots {
        for col in [-1, 0, 1] {
            for row in [-1, 0, 1] {
                if col == 0 && row == 0 {
                    continue;
                }
                let position = Point {
                    row: robot.position.row + row,
                    col: robot.position.col + col,
                };
                if distinct_positions.contains(&position) {
                    neighbors += 1;
                }
            }
        }
    }
    neighbors
}

fn part_two(robots: Vec<Robot>, max_row: i32, max_col: i32, steps: i32) {
    let mut interest_measurements = vec![(0, 0); steps as usize];
    let mut current_robots = robots.clone();
    for i in 0..steps {
        interest_measurements[i as usize] = (i, interest(&current_robots));
        current_robots = current_robots
            .iter()
            .map(|robot| robot.step(max_row, max_col))
            .collect();
    }
    // Order the emptiness measurements by emptiness descending first, then by step number ascending
    interest_measurements.sort_by_key(|(step, emptiness)| (-*emptiness, *step));
    for (step, i) in interest_measurements.iter().take(1000) {
        println!("Step {:?} -> Interest: {:?}", step, i);
        current_robots = robots.clone();
        for _ in 0..*step {
            current_robots = current_robots
                .iter()
                .map(|robot| robot.step(max_row, max_col))
                .collect();
        }
        visualize(&current_robots, max_row, max_col, "");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_move() {
        let robots = read_input("move.txt");
        let mut robot = robots[0].clone();
        let quadrants = [
            Quadrant::BottomLeft,
            Quadrant::TopLeft,
            Quadrant::BottomRight,
            Quadrant::TopRight,
        ];
        for (i, q) in quadrants.iter().enumerate() {
            println!("{}: Robot: {:?}, Quadrant: {:?}", i, robot.position, *q);
            assert_eq!(get_quadrant(robot, 7, 11), Some(*q));
            robot = robot.step(7, 11);
        }
    }

    #[test]
    fn test_part_one() {
        let robots = read_input("test.txt");
        let result = part_one(robots, 7, 11);
        assert_eq!(result, 12);
    }
}

fn main() {
    let robots = read_input("input.txt");
    println!("Part One: {}", part_one(robots.clone(), 103, 101));
    // You'll need to debug this function and pause at the inspect step or dump it into a large text file to inspect it. The right answer should be within one or two "interesting" pictures.
    part_two(robots, 103, 101, 10000);
}
