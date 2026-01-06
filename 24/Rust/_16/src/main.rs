use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
    fn inverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
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
}

fn read_input(path: &str) -> MazeProblem {
    let input = fs::read_to_string(path).expect("Failed to read input file");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut start: Option<Position> = None;
    let mut end: Option<Point> = None;
    let mut walls = HashSet::new();
    let turn_penalty = -1000;
    let move_penalty = -1;
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

// fn render_maze(walls: &HashSet<Point>, start: &Point, end: &Point) {
//     let max_row = walls.iter().map(|p| p.row).max().unwrap() + 1;
//     let max_col = walls.iter().map(|p| p.col).max().unwrap() + 1;
//     let mut maze = vec![vec!['.'; max_col]; max_row];
//     for wall in walls {
//         maze[wall.row][wall.col] = '#';
//     }
//     maze[start.row][start.col] = 'S';
//     maze[end.row][end.col] = 'E';
//     for row in maze {
//         println!("{}", row.iter().collect::<String>());
//     }
// }

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

fn flood_fill(
    problem: &MazeProblem,
    starting_positions: &Vec<Position>,
    penalty_map: Option<HashMap<Position, i32>>,
    max_penalty: i32,
) -> HashMap<Position, i32> {
    let mut queue = BinaryHeap::new();
    for starting_position in starting_positions {
        queue.push(State {
            position: starting_position.clone(),
            cost: 0,
        });
    }
    let skip_all_empty_penalties = penalty_map.is_some();
    let penalties = penalty_map.unwrap_or(HashMap::new());
    let mut visited = HashMap::new();
    while let Some(current) = queue.pop() {
        let cost_from_other_direction = penalties.get(&current.position);
        if cost_from_other_direction.is_none() && skip_all_empty_penalties {
            continue;
        }
        if current.cost.abs() + cost_from_other_direction.unwrap_or(&0) > max_penalty {
            continue;
        }
        let existing_cost = visited.get(&current.position);
        if existing_cost.is_some() && *existing_cost.unwrap() <= current.cost {
            continue;
        }
        visited.insert(current.position.clone(), current.cost);
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
        if !problem.walls.contains(&forward.point) {
            let forward_cost = current.cost - problem.move_penalty;
            queue.push(State {
                position: forward,
                cost: forward_cost,
            });
        }
        if !problem.walls.contains(&left.point.move_by(&left.direction)) {
            let left_cost = current.cost - problem.turn_penalty;
            queue.push(State {
                position: left,
                cost: left_cost,
            });
        }
        if !problem
            .walls
            .contains(&right.point.move_by(&right.direction))
        {
            let right_cost = current.cost - problem.turn_penalty;
            queue.push(State {
                position: right,
                cost: right_cost,
            });
        }
    }
    visited
}

fn inverse_directions(penalty_map: &HashMap<Position, i32>) -> HashMap<Position, i32> {
    let mut inverse_penalty_map = HashMap::new();
    for (position, penalty) in penalty_map.iter() {
        inverse_penalty_map.insert(
            Position {
                point: position.point.clone(),
                direction: position.direction.inverse(),
            },
            *penalty,
        );
    }
    inverse_penalty_map
}

fn part_two(problem: &MazeProblem, max_penalty: usize) -> usize {
    let reachable_from_start = flood_fill(
        problem,
        &vec![problem.start.clone()],
        None,
        max_penalty as i32,
    );
    // Detect which directions we're allowed to start from in the reverse direction. Check the problem.end point with all 4 possible directions.
    // If we can reach (row, col) from Up in the first flood_fill, we should be able to reach (row, col) from Down in the reverse flood fill
    let inverse_penalty_map = inverse_directions(&reachable_from_start);
    let mut end_positions = Vec::new();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    for direction in directions {
        if reachable_from_start.get(&Position {
            point: problem.end,
            direction: direction.clone(),
        }).is_none() {
            continue;
        }
        end_positions.push(Position {
            point: problem.end,
            direction: direction.inverse(),
        });
    }
    let reachable_from_end = flood_fill(
        problem,
        &end_positions,
        Some(inverse_penalty_map),
        max_penalty as i32,
    );

    let reachable_points = reachable_from_end
        .iter()
        .map(|(p, _c)| {
            p.point
        })
        .collect();
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
