use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    point: Point,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Point {
    fn move_by(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Point {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Point {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Point {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

struct MazeProblem {
    start: Position,
    end: Point,
    walls: HashSet<Point>,
    turn_penalty: i32,
    move_penalty: i32,
    heuristic: fn(&Point, &Point) -> i32,
}

fn read_input(path: &str) -> MazeProblem {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut start: Option<Position> = None;
    let mut end: Option<Point> = None;
    let mut walls = HashSet::new();
    let turn_penalty = -1000;
    let move_penalty = -1;
    let heuristic = |a: &Point, b: &Point| {
        (a.row as i32 - b.row as i32).abs() + (a.col as i32 - b.col as i32).abs()
    };
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert(Point { row, col });
            }
            if c == 'S' {
                start = Some(Position {
                    point: Point { row, col },
                    direction: Direction::Right,
                });
            }
            if c == 'E' {
                end = Some(Point { row, col });
            }
        }
    }
    MazeProblem {
        start: start.unwrap(),
        end: end.unwrap(),
        walls,
        turn_penalty,
        move_penalty,
        heuristic,
    }
}

struct State {
    position: Position,
    cost: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn render_maze(walls: &HashSet<Point>, start: &Point, end: &Point) {
    let max_row = walls.iter().map(|p| p.row).max().unwrap() + 1;
    let max_col = walls.iter().map(|p| p.col).max().unwrap() + 1;
    let mut maze = vec![vec!['.'; max_col]; max_row];
    for wall in walls {
        maze[wall.row][wall.col] = '#';
    }
    maze[start.row][start.col] = 'S';
    maze[end.row][end.col] = 'E';
    for row in maze {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part_one(problem: &MazeProblem) -> usize {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        position: problem.start.clone(),
        cost: 0,
    });
    let mut visited = HashSet::<Position>::new();
    while let Some(current) = priority_queue.pop() {
        if current.position.point == problem.end {
            return current.cost.abs() as usize;
        }
        if visited.contains(&current.position) {
            continue;
        }
        // println!(
        //     "Current score ({}): {:?} -> {:?}",
        //     current.cost.abs(), current.position.point, current.position.direction
        // );
        // render_maze(&problem.walls, &current.position.point, &problem.end);
        visited.insert(current.position.clone());
        let direction = current.position.direction;
        let forward = Position {
            point: current.position.point.move_by(&direction),
            direction: direction.clone(),
        };
        let left = Position {
            point: current.position.point.clone(),
            direction: direction.clone().left(),
        };
        let right = Position {
            point: current.position.point.clone(),
            direction: direction.right(),
        };
        if !problem.walls.contains(&forward.point) && !visited.contains(&forward) {
            let forward_cost = current.cost - problem.move_penalty;
            priority_queue.push(State {
                position: Position {
                    point: forward.point,
                    direction: direction,
                },
                cost: forward_cost,
            });
        }
        if !problem.walls.contains(&left.point.move_by(&left.direction)) && !visited.contains(&left)
        {
            let left_cost = current.cost - problem.turn_penalty;
            priority_queue.push(State {
                position: left,
                cost: left_cost,
            });
        }
        if !problem
            .walls
            .contains(&right.point.move_by(&right.direction))
            && !visited.contains(&right)
        {
            let right_cost = current.cost - problem.turn_penalty;
            priority_queue.push(State {
                position: right,
                cost: right_cost,
            });
        }
    }
    usize::MAX
}

struct Outcome {
    min_cost: usize,
    direction: Direction,
}

fn min_cost_to_reach(
    problem: &MazeProblem,
    start: &Position,
    end: &Point,
    max_penalty: i32,
) -> Vec<Outcome> {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        position: start.clone(),
        cost: 0,
    });
    let mut possible_directions = Vec::new();
    let mut visited = HashSet::<Position>::new();
    let mut min_cost_to_end: Option<usize> = None;
    while let Some(current) = priority_queue.pop() {
        if current.position.point == *end {
            let cost = current.cost.abs() as usize;
            if min_cost_to_end.is_none() || cost == min_cost_to_end.unwrap() {
                min_cost_to_end = Some(cost);
                possible_directions.push(Outcome {
                    min_cost: cost,
                    direction: current.position.direction,
                });
            }
            continue;
        }
        if current.cost.abs() > max_penalty {
            continue;
        }
        if visited.contains(&current.position) {
            continue;
        }
        visited.insert(current.position.clone());
        let direction = current.position.direction;
        let forward = Position {
            point: current.position.point.move_by(&direction),
            direction: direction.clone(),
        };
        let left = Position {
            point: current.position.point.clone(),
            direction: direction.clone().left(),
        };
        let right = Position {
            point: current.position.point.clone(),
            direction: direction.right(),
        };
        if !problem.walls.contains(&forward.point) && !visited.contains(&forward) {
            let forward_cost = current.cost - problem.move_penalty;
            if forward_cost.abs() <= max_penalty {
                priority_queue.push(State {
                    position: Position {
                        point: forward.point,
                        direction: direction,
                    },
                    cost: forward_cost,
                });
            }
        }
        if !problem.walls.contains(&left.point.move_by(&left.direction)) && !visited.contains(&left)
        {
            let left_cost = current.cost - problem.turn_penalty;
            if left_cost.abs() <= max_penalty {
                priority_queue.push(State {
                    position: left,
                    cost: left_cost,
                });
            }
        }
        if !problem
            .walls
            .contains(&right.point.move_by(&right.direction))
            && !visited.contains(&right)
        {
            let right_cost = current.cost - problem.turn_penalty;
            if right_cost.abs() <= max_penalty {
                priority_queue.push(State {
                    position: right,
                    cost: right_cost,
                });
            }
        }
    }
    possible_directions
}

fn render_reachable_points(problem: &MazeProblem, reachable_points: &HashSet<Point>) {
  let max_row = problem.walls.iter().map(|p| p.row).max().unwrap() + 1;
  let max_col = problem.walls.iter().map(|p| p.col).max().unwrap() + 1;
  let mut maze = vec![vec!['.'; max_col]; max_row];
  for wall in problem.walls.iter() {
      maze[wall.row][wall.col] = '#';
  }
  for reachable_point in reachable_points {
    maze[reachable_point.row][reachable_point.col] = 'O';
  }
  for row in maze {
      println!("{}", row.iter().collect::<String>());
  }
}

fn part_two(problem: &MazeProblem, max_penalty: usize) -> usize {
    let max_row = problem.walls.iter().map(|p| p.row).max().unwrap() + 1;
    let max_col = problem.walls.iter().map(|p| p.col).max().unwrap() + 1;
    let mut reachable_points = HashSet::new();
    for row in 0..max_row {
        for col in 0..max_col {
            let point = Point { row, col };
            if problem.walls.contains(&point) {
                continue;
            }
            if point == problem.start.point || point == problem.end {
                // println!("Point {:?} is start or end", point);
                reachable_points.insert(point);
                continue;
            }
            let start_to_point_outcome =
                min_cost_to_reach(problem, &problem.start, &point, max_penalty as i32);
            if start_to_point_outcome.len() == 0 {
                // println!("Point {:?} is unreachable from start", point);
                continue;
            }
            // Test the reachability from all possible directions
            for outcome in start_to_point_outcome {
              let direction = outcome.direction;
              let new_position = Position {
                point,
                direction,
              };
              let remaining_cost = max_penalty as i32 - outcome.min_cost as i32;
              let point_to_end_outcome = min_cost_to_reach(
                  problem,
                  &new_position,
                  &problem.end,
                  remaining_cost,
              );
              // Check if the minimum cost path from point to end makes the total equal to optimal
              if let Some(min_end_cost) = point_to_end_outcome.iter().map(|o| o.min_cost).min() {
                if outcome.min_cost + min_end_cost == max_penalty {
                  reachable_points.insert(point);
                  break;
                }
              }
            }
        }
    }
    render_reachable_points(problem, &reachable_points);
    reachable_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(&problem), 7036);
    }
    #[test]
    fn test_part_two() {
        let problem = read_input("test.txt");
        assert_eq!(part_two(&problem, 7036), 45);
        let problem = read_input("test2.txt");
        assert_eq!(part_two(&problem, 11048), 64);
    }
}

fn main() {
    let problem = read_input("input.txt");
    let max_penalty = part_one(&problem);
    println!("Part one: {}", max_penalty);
    println!("Part two: {}", part_two(&problem, max_penalty));
}
