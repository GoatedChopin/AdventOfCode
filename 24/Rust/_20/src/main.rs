use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

struct Problem {
    walls: HashSet<Point>,
    max_row: i32,
    max_col: i32,
    start: Point,
    end: Point,
}

fn read_input(file_path: &str) -> Problem {
    let lines = fs::read_to_string(file_path).expect("Failed to read input file");
    let lines = lines.lines().collect::<Vec<&str>>();
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert(Point {
                    row: row as i32,
                    col: col as i32,
                });
            } else if c == 'S' {
                start = Some(Point {
                    row: row as i32,
                    col: col as i32,
                });
            } else if c == 'E' {
                end = Some(Point {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }
    Problem {
        walls,
        max_row: lines.len() as i32,
        max_col: lines[0].len() as i32,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn in_bounds(point: &Point, problem: &Problem) -> bool {
    point.row >= 0 && point.row < problem.max_row && point.col >= 0 && point.col < problem.max_col
}

fn get_neighbors(point: &Point, problem: &Problem) -> Vec<Point> {
    vec![
        Point {
            row: point.row - 1,
            col: point.col,
        },
        Point {
            row: point.row + 1,
            col: point.col,
        },
        Point {
            row: point.row,
            col: point.col - 1,
        },
        Point {
            row: point.row,
            col: point.col + 1,
        },
    ]
    .into_iter()
    .filter(|p| in_bounds(p, problem))
    .collect()
}

fn get_cheat_points(point: &Point, problem: &Problem) -> Vec<Point> {
    let walls: Vec<bool> = vec![
        Point {
            row: point.row - 1,
            col: point.col,
        },
        Point {
            row: point.row + 1,
            col: point.col,
        },
        Point {
            row: point.row,
            col: point.col + 1,
        },
        Point {
            row: point.row,
            col: point.col - 1,
        },
    ]
    .into_iter()
    .map(|p| problem.walls.contains(&p))
    .collect();
    vec![
        Point {
            row: point.row - 2,
            col: point.col,
        },
        Point {
            row: point.row + 2,
            col: point.col,
        },
        Point {
            row: point.row,
            col: point.col + 2,
        },
        Point {
            row: point.row,
            col: point.col - 2,
        },
    ]
    .into_iter()
    .zip(walls.iter())
    .filter(|(p, wall)| **wall && in_bounds(p, problem))
    .filter(|(p, _)| !problem.walls.contains(p))
    .map(|(p, _)| p)
    .collect()
}

struct SearchState {
    point: Point,
    steps: i32,
}

fn midpoint(point1: &Point, point2: &Point) -> Point {
    let difference = Point {
        row: point2.row - point1.row,
        col: point2.col - point1.col,
    };
    Point {
        row: point1.row + difference.row / 2,
        col: point1.col + difference.col / 2,
    }
}

fn direction_char(point1: &Point, point2: &Point) -> char {
    let difference = Point {
        row: point2.row - point1.row,
        col: point2.col - point1.col,
    };
    if difference.row > 0 {
        'v'
    } else if difference.row < 0 {
        '^'
    } else if difference.col > 0 {
        '>'
    } else {
        '<'
    }
}

fn render_skips(problem: &Problem, good_skips: &HashSet<(Point, Point)>) {
    let mut skips_to_render = HashMap::new();
    good_skips.iter().for_each(|(p1, p2)| {
        skips_to_render.insert(midpoint(p1, p2), direction_char(p1, p2));
    });
    for row in 0..problem.max_row {
        for col in 0..problem.max_col {
            let point = Point { row, col };
            if skips_to_render.contains_key(&point) {
                print!("{}", skips_to_render.get(&point).unwrap());
            } else if problem.walls.contains(&point) {
                print!("#");
            } else if problem.start == point {
                print!("S");
            } else if problem.end == point {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_one(problem: &Problem, min_saved_steps: i32) -> usize {
    // Step 1: flood fill the map from the end point to every other point.
    let mut queue = VecDeque::new();
    let mut distance_from_end = HashMap::new();
    queue.push_back(SearchState {
        point: problem.end,
        steps: 0,
    });
    while let Some(current) = queue.pop_front() {
        if distance_from_end.contains_key(&current.point) {
            continue;
        }
        distance_from_end.insert(current.point, current.steps);
        for neighbor in get_neighbors(&current.point, problem) {
            if !in_bounds(&neighbor, problem) || problem.walls.contains(&neighbor) {
                continue;
            }
            queue.push_back(SearchState {
                point: neighbor,
                steps: current.steps + 1,
            });
        }
    }

    // Benchmark by capturing the distance from end to start
    let min_fair_distance = *distance_from_end.get(&problem.start).unwrap();

    // Flood fill from start to everywhere else. Anytime we could walk into a wall,
    // we need to see if the step beyond the wall is in the "distance_from_end",
    // and if that (distance_from_end + current.steps) < (min_fair_distance - 100)
    let mut good_skips = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(SearchState {
        point: problem.start,
        steps: 0,
    });
    let mut visited = HashSet::new();
    while let Some(current) = queue.pop_front() {
        if current.steps > min_fair_distance - min_saved_steps {
            continue;
        }
        if visited.contains(&current.point) {
            continue;
        }
        visited.insert(current.point);
        for neighbor in get_neighbors(&current.point, problem) {
            if !in_bounds(&neighbor, problem) || problem.walls.contains(&neighbor) {
                continue;
            }
            queue.push_back(SearchState {
                point: neighbor,
                steps: current.steps + 1,
            });
        }
        for cheat_point in get_cheat_points(&current.point, problem) {
            if !distance_from_end.contains_key(&cheat_point) {
                continue;
            }
            if distance_from_end.get(&cheat_point).unwrap() + current.steps + min_saved_steps + 2
                <= min_fair_distance
            {
                good_skips.insert((current.point, cheat_point));
            }
        }
    }

    render_skips(&problem, &good_skips);
    good_skips.len()
}

fn manhattan_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.row - point2.row).abs() + (point1.col - point2.col).abs()
}

fn render_skip_starts(problem: &Problem, good_skips: &HashSet<Point>) {
    for row in 0..problem.max_row {
        for col in 0..problem.max_col {
            let point = Point { row, col };
            if good_skips.contains(&point) {
                print!("?");
            } else if problem.walls.contains(&point) {
                print!("#");
            } else if problem.start == point {
                print!("S");
            } else if problem.end == point {
                print!("E");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_two(problem: &Problem, min_saved_steps: i32) -> usize {
    // Step 1: flood fill the map from the end point to every other point.
    let mut queue = VecDeque::new();
    let mut distance_from_end = HashMap::new();
    queue.push_back(SearchState {
        point: problem.end,
        steps: 0 as i32,
    });
    while let Some(current) = queue.pop_front() {
        if distance_from_end.contains_key(&current.point) {
            continue;
        }
        distance_from_end.insert(current.point, current.steps);
        for neighbor in get_neighbors(&current.point, problem) {
            if !in_bounds(&neighbor, problem) || problem.walls.contains(&neighbor) {
                continue;
            }
            queue.push_back(SearchState {
                point: neighbor,
                steps: current.steps + 1,
            });
        }
    }

    // Benchmark by capturing the distance from end to start
    let min_fair_distance = *distance_from_end.get(&problem.start).unwrap();

    let points_of_interest = distance_from_end
        .iter()
        .map(|(point, steps)| (point, *steps))
        .filter(|(point, _)| *point != &problem.end)
        .filter(|(_, steps)| *steps <= min_fair_distance - min_saved_steps)
        .collect::<Vec<_>>();

    // Flood fill from start to everywhere else. Anytime we could walk into a wall,
    // we need to see if the step beyond the wall is in the "distance_from_end",
    // and if that (distance_from_end + current.steps) < (min_fair_distance - 100)
    let mut good_skips = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(SearchState {
        point: problem.start,
        steps: 0,
    });
    let mut visited = HashSet::new();
    while let Some(current) = queue.pop_front() {
        if current.steps > min_fair_distance - min_saved_steps {
            continue;
        }
        if visited.contains(&current.point) {
            continue;
        }
        visited.insert(current.point);
        for neighbor in get_neighbors(&current.point, problem) {
            if !in_bounds(&neighbor, problem) || problem.walls.contains(&neighbor) {
                continue;
            }
            queue.push_back(SearchState {
                point: neighbor,
                steps: current.steps + 1,
            });
        }
        let dist_to_end = manhattan_distance(&current.point, &problem.end);
        if dist_to_end <= 20
            && current.steps + dist_to_end + min_saved_steps <= min_fair_distance
        {
            good_skips.insert((current.point, problem.end));
        }
        points_of_interest
            .iter()
            .filter(|(point, _)| manhattan_distance(point, &current.point) <= 20)
            .filter(|(point, steps_to_end)| {
                let new_cost = current.steps
                    + manhattan_distance(point, &current.point)
                    + *steps_to_end
                    + min_saved_steps;
                let is_save = new_cost <= min_fair_distance;
                // if is_save {
                //     println!(
                //         "Saving enough steps from {:?} to {:?} with {} steps to spare",
                //         current.point,
                //         point,
                //         min_fair_distance - new_cost,
                //     );
                // }
                is_save
            })
            .for_each(|(point, _)| {
                good_skips.insert((current.point, (*point).clone()));
            });
    }
    render_skip_starts(
        &problem,
        &good_skips.iter().map(|(p1, _)| p1.clone()).collect(),
    );
    good_skips.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(&problem, 64), 1);
        assert_eq!(part_one(&problem, 40), 2);
    }

    #[test]
    fn test_part_two() {
        let problem = read_input("test.txt");
        assert_eq!(part_two(&problem, 76), 3);
        assert_eq!(part_two(&problem, 74), 7);
        assert_eq!(part_two(&problem, 72), 29);
        assert_eq!(part_two(&problem, 70), 41);
        assert_eq!(part_two(&problem, 68), 55);
        assert_eq!(part_two(&problem, 66), 67);
        assert_eq!(part_two(&problem, 64), 67 + 19);
        assert_eq!(part_two(&problem, 62), 67 + 19 + 20);
        assert_eq!(part_two(&problem, 60), 67 + 19 + 20 + 23);
        assert_eq!(part_two(&problem, 58), 67 + 19 + 20 + 23 + 25);
        assert_eq!(part_two(&problem, 56), 67 + 19 + 20 + 23 + 25 + 39);
        assert_eq!(part_two(&problem, 54), 67 + 19 + 20 + 23 + 25 + 39 + 29);
        assert_eq!(part_two(&problem, 52), 67 + 19 + 20 + 23 + 25 + 39 + 29 + 31);
        assert_eq!(part_two(&problem, 50), 32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3);
    }
}

fn main() {
    let problem = read_input("input.txt");
    println!("{}", part_one(&problem, 100));
    println!("{}", part_two(&problem, 100));
}
