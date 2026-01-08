use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let parts: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        Self {
            row: parts[1],
            col: parts[0],
        }
    }
    fn inverse(self) -> Self {
        Self {
            row: self.col,
            col: self.row,
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

fn get_obstacles<'a>(
    problem: &Problem,
    steps: i32,
    past_obstacles: &'a mut HashMap<i32, HashSet<Point>>,
) -> &'a HashSet<Point> {
    if past_obstacles.contains_key(&steps) {
        return &past_obstacles[&steps];
    }
    past_obstacles.insert(
        steps,
        problem
            .falling_bytes
            .iter()
            .take((steps) as usize)
            .cloned()
            .collect::<HashSet<Point>>(),
    );
    &past_obstacles[&steps]
}

#[derive(Debug, Copy, Clone)]
struct SearchState {
    position: Point,
    steps: i32,
    distance: i32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize by steps (actual cost) - Dijkstra's algorithm
        // Smaller steps = higher priority (reversed for min heap)
        other.steps.cmp(&self.steps)
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

fn render_state(state: &SearchState, problem: &Problem, min_bytes_fallen: i32) {
    let mut grid = vec![
        vec!['.'; (problem.dimensions.col + 1) as usize];
        (problem.dimensions.row + 1) as usize
    ];
    for obstacle in problem.falling_bytes.iter().take(min_bytes_fallen as usize) {
        grid[obstacle.row as usize][obstacle.col as usize] = '#';
    }
    grid[state.position.row as usize][state.position.col as usize] = 'o';
    println!("Steps: {}", state.steps);
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn part_one(problem: &Problem, min_bytes_fallen: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push(SearchState {
        position: Point { row: 0, col: 0 },
        steps: 0,
        distance: 0,
    });
    let mut visited = HashSet::new();
    visited.insert(Point { row: 0, col: 0 });
    let mut past_obstacles = HashMap::new();
    while let Some(state) = heap.pop() {
        if state.position == problem.dimensions {
          // render_state(
          //   &SearchState {
          //     position: problem.dimensions,
          //     steps: min_bytes_fallen,
          //     distance: 0,
          //   },
          //   &problem,
          //   min_bytes_fallen,
          // );
          return state.steps;
        }
        // render_state(&state, &problem, 25);
        for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_position = Point {
                row: state.position.row + direction.0,
                col: state.position.col + direction.1,
            };
            if visited.contains(&new_position) {
                continue;
            }
            visited.insert(new_position);
            if !in_bounds(new_position, problem.dimensions) {
                continue;
            }
            let obstacles = get_obstacles(&problem, min_bytes_fallen, &mut past_obstacles);
            if obstacles.contains(&new_position) {
                continue;
            }
            let distance = manhattan_distance(new_position, problem.dimensions);
            let new_state = SearchState {
                position: new_position,
                steps: state.steps + 1,
                distance,
            };
            heap.push(new_state);
        }
    }
    return -1;
}

fn part_two(problem: &Problem) -> Point {
    let mut min_bytes_fallen = 0;
    while min_bytes_fallen < problem.falling_bytes.len() {
        if part_one(problem, min_bytes_fallen as i32) == -1 {
            return problem.falling_bytes[min_bytes_fallen - 1].inverse();
        }
        min_bytes_fallen += 1;
    }
    return Point {row: 0, col: 0};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_obstacles() {
        let problem = read_input("test.txt");
        let mut hash_map = HashMap::new();
        let obstacles = get_obstacles(&problem, 12, &mut hash_map);
        assert_eq!(obstacles.len(), 12);
    }

    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(&problem, 12), 22);
    }

    #[test]
    fn test_part_two() {
        let problem = read_input("test.txt");
        assert_eq!(part_two(&problem), Point {row: 6, col: 1});
    }
}

fn main() {
    let problem = read_input("input.txt");
    println!("Part one: {}", part_one(&problem, 1024));
    println!("Part two: {:?}", part_two(&problem));
}
