use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let parts: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        Self {
            row: parts[0],
            col: parts[1],
        }
    }
}

struct Problem {
    falling_bytes: Vec<Point>,
    dimensions: Point,
}

fn read_input(path: &str) -> Problem {
    let input = fs::read_to_string(path).unwrap();
    let lines = input.lines();
    let falling_bytes: Vec<Point> = lines.map(Point::from_str).collect();
    let max_dim = falling_bytes
        .iter()
        .fold(Point { row: 0, col: 0 }, |a, b| Point {
            row: a.row.max(b.row),
            col: a.col.max(b.col),
        });
    let dimensions = Point {
        row: max_dim.row.max(max_dim.col),
        col: max_dim.col.max(max_dim.row),
    };
    Problem {
        falling_bytes,
        dimensions,
    }
}

fn in_bounds(position: Point, dimensions: Point) -> bool {
    position.row >= 0
        && position.row <= dimensions.row
        && position.col >= 0
        && position.col <= dimensions.col
}

fn manhattan_distance(position: Point, dimensions: Point) -> i32 {
    (position.row - dimensions.row).abs() + (position.col - dimensions.col).abs()
}

fn get_obstacles<'a>(problem: &Problem, steps: i32, past_obstacles: &'a mut HashMap<i32, HashSet<Point>>) -> &'a HashSet<Point> {
    if past_obstacles.contains_key(&steps) {
        return &past_obstacles[&steps];
    }
    past_obstacles.insert(steps, problem.falling_bytes.iter().take(steps as usize).cloned().collect::<HashSet<Point>>());
    &past_obstacles[&steps]
}

struct SearchState {
    position: Point,
    steps: i32,
    heuristic: i32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.steps == other.steps
    }
}

impl Eq for SearchState {}

fn render_state(state: &SearchState, problem: &Problem) {
  let mut grid = vec![vec!['.'; (problem.dimensions.col + 1) as usize]; (problem.dimensions.row + 1) as usize];
  for obstacle in problem.falling_bytes.iter().take(state.steps as usize) {
    grid[obstacle.row as usize][obstacle.col as usize] = '#';
  }
  grid[state.position.row as usize][state.position.col as usize] = 'o';
  for row in grid.iter() {
    println!("{}", row.iter().collect::<String>());
  }
  println!();
}

fn part_one(problem: &Problem) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(SearchState {
        position: Point { row: 0, col: 0 },
        steps: 0,
        heuristic: 0,
    });
    let mut past_obstacles = HashMap::new();
    while let Some(state) = heap.pop() {
        if state.position == problem.dimensions {
            render_state(&state, &problem);
            return state.steps;
        }
        render_state(&state, &problem);
        for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_position = Point {
                row: state.position.row + direction.0,
                col: state.position.col + direction.1,
            };
            if !in_bounds(new_position, problem.dimensions) {
                continue;
            }
            let obstacles = get_obstacles(&problem, state.steps + 1, &mut past_obstacles);
            if obstacles.contains(&new_position) {
                continue;
            }
            let heuristic = -1 * (state.steps + manhattan_distance(new_position, problem.dimensions));
            let new_state = SearchState {
                position: new_position,
                steps: state.steps + 1,
                heuristic,
            };
            heap.push(new_state);
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(&problem), 22);
    }
}
fn main() {
    println!("Hello, world!");
}
