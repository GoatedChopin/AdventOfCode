use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
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

#[derive(Eq, Hash, PartialEq, Clone)]
struct Box {
    position: Point,
}

impl Box {
    fn gps(&self) -> usize {
        self.position.row * 100 + self.position.col
    }
}

#[derive(Clone)]
struct Robot {
    position: Point,
}

// impl Robot {
//     fn move_by(&mut self, direction: Direction) {
//         match direction {
//             Direction::Up => self.position.row -= 1,
//             Direction::Down => self.position.row += 1,
//             Direction::Left => self.position.col -= 1,
//             Direction::Right => self.position.col += 1,
//         }
//     }
// }

#[derive(Clone)]
struct Problem {
    robot: Robot,
    boxes: HashSet<Box>,
    walls: HashSet<Point>,
    instructions: Vec<Direction>,
    links: HashMap<Point, Point>,
}

struct Outcome {
    robot_position: Point,
    box_start_position: Option<Point>,
    box_end_position: Option<Point>,
}

struct PartTwoOutcome {
    robot_position: Point,
    boxes_to_remove: HashSet<Box>,
    boxes_to_add: HashSet<Box>,
    links_to_add: HashSet<(Point, Point)>,
}

impl Problem {
    fn move_robot(&mut self, instruction: &Direction) -> Outcome {
        let new_position = self.robot.position.move_by(&instruction);
        if self.walls.contains(&new_position) {
            return Outcome {
                robot_position: self.robot.position,
                box_start_position: None,
                box_end_position: None,
            };
        }
        if self.boxes.contains(&Box {
            position: new_position,
        }) {
            // Check to see if we can move the box
            let mut box_position = new_position.clone();
            box_position = box_position.move_by(&instruction);
            while !self.walls.contains(&box_position)
                && self.boxes.contains(&Box {
                    position: box_position,
                })
            {
                box_position = box_position.move_by(&instruction);
            }
            if self.walls.contains(&box_position) {
                return Outcome {
                    robot_position: self.robot.position,
                    box_start_position: None,
                    box_end_position: None,
                };
            }
            return Outcome {
                robot_position: new_position,
                box_start_position: Some(new_position),
                box_end_position: Some(box_position),
            };
        }
        Outcome {
            robot_position: new_position,
            box_start_position: None,
            box_end_position: None,
        }
    }

    fn can_move_box(&self, position: Point, instruction: &Direction) -> (bool, HashSet<(Point, Point)>) {
        let mut boxes_to_remove = HashSet::new();
        let new_position = position.move_by(&instruction);
        let linked_position = self.links.get(&position).unwrap();
        let new_linked_position = linked_position.move_by(&instruction);
        if self.walls.contains(&new_position) || self.walls.contains(&new_linked_position) {
            return (false, boxes_to_remove);
        }
        boxes_to_remove.insert((position, *linked_position));
        let mut can_move = true;
        if self.boxes.contains(&Box {
            position: new_position,
        }) {
          let (og_can_move, og_boxes_to_remove) = self.can_move_box(new_position, instruction);
          can_move = can_move && og_can_move;
          boxes_to_remove.extend(og_boxes_to_remove);
        }
        if self.boxes.contains(&Box {
            position: new_linked_position,
        }) {
          let (linked_can_move, linked_boxes_to_remove) = self.can_move_box(new_linked_position, instruction);
          can_move = can_move && linked_can_move;
          boxes_to_remove.extend(linked_boxes_to_remove);
        }
        (can_move, boxes_to_remove)
    }

    // fn infer_links(&self, points: HashSet<Point>) -> HashMap<Point, Point> {
    //     let mut links = HashMap::new();
    //     let mut ordered_points = points.iter().collect::<Vec<&Point>>();
    //     ordered_points.sort_by_key(|p| (p.row, p.col));
    //     for i in 0..ordered_points.len() - 1 {
    //         let point = ordered_points[i];
    //         let next_point = ordered_points[i + 1];
    //         links.insert(*point, *next_point);
    //     }
    //     links
    // }

    fn move_robot_part_two(
        &mut self,
        position: Point,
        instruction: &Direction,
    ) -> PartTwoOutcome {
        let new_position = position.move_by(&instruction);
        if self.walls.contains(&new_position) {
            return PartTwoOutcome {
                robot_position: self.robot.position,
                boxes_to_remove: HashSet::new(),
                boxes_to_add: HashSet::new(),
                links_to_add: HashSet::new(),
            };
        }
        if self.boxes.contains(&Box {
            position: new_position,
        }) {
            // Check to see if we can move the box
            let box_position = new_position.clone();
            let (can_move, boxes_to_remove) = self.can_move_box(box_position, instruction);
            if !can_move {
                return PartTwoOutcome {
                    robot_position: self.robot.position,
                    boxes_to_remove: HashSet::new(),
                    boxes_to_add: HashSet::new(),
                    links_to_add: HashSet::new(),
                };
            }

            let mut boxes_to_add = HashSet::new();
            let mut flat_boxes_to_remove = HashSet::new();
            let mut links_to_remove = HashSet::new();
            boxes_to_remove.iter().for_each(|(box_position, linked_box_position)| {
                links_to_remove.insert((*box_position, *linked_box_position));
                flat_boxes_to_remove.insert(Box {
                    position: *box_position,
                });
                flat_boxes_to_remove.insert(Box {
                    position: *linked_box_position,
                });
                boxes_to_add.insert(Box {
                    position: box_position.move_by(instruction),
                });
                boxes_to_add.insert(Box {
                    position: linked_box_position.move_by(instruction),
                });
            });

            let links_to_add = boxes_to_remove.iter().map(|(box_position, linked_box_position)| (box_position.move_by(instruction), linked_box_position.move_by(instruction))).collect::<HashSet<(Point, Point)>>();
            return PartTwoOutcome {
                robot_position: new_position,
                boxes_to_remove: flat_boxes_to_remove,
                boxes_to_add: boxes_to_add,
                links_to_add: links_to_add,
            };
        }
        PartTwoOutcome {
            robot_position: new_position,
            boxes_to_remove: HashSet::new(),
            boxes_to_add: HashSet::new(),
            links_to_add: HashSet::new(),
        }
    }

    fn render(&self) {
        let mut max_row = 0;
        let mut max_col = 0;
        for point in self.walls.iter() {
            if point.row > max_row {
                max_row = point.row;
            }
            if point.col > max_col {
                max_col = point.col;
            }
        }
        for row in 0..=max_row {
            for col in 0..=max_col {
                if self.walls.contains(&Point { row, col }) {
                    print!("#");
                } else if self.robot.position == (Point { row, col }) {
                    print!("@");
                } else if self.boxes.contains(&Box {
                    position: Point { row, col },
                }) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn read_input(path: &str) -> Problem {
    let mut robot = Robot {
        position: Point { row: 0, col: 0 },
    };
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    let mut instructions = Vec::new();
    for (row, line) in fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
    {
        for (col, c) in line.chars().enumerate() {
            if c == '@' {
                robot.position = Point { row, col };
            }
            if c == 'O' {
                boxes.insert(Box {
                    position: Point { row, col },
                });
            }
            if c == '#' {
                walls.insert(Point { row, col });
            }
            if let Some(direction) = Direction::from_char(c) {
                instructions.push(direction);
            }
        }
    }
    Problem {
        robot,
        boxes,
        walls,
        instructions,
        links: HashMap::new(),
    }
}

// All walls and boxes are double-width in the part two puzzle.
fn read_input_part_two(path: &str) -> Problem {
    let mut robot = Robot {
        position: Point { row: 0, col: 0 },
    };
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    let mut instructions = Vec::new();
    let mut links = HashMap::new();
    for (row, line) in fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .enumerate()
    {
        let mut col = 0;
        for c in line.chars() {
            if c == '@' {
                robot.position = Point { row, col };
            }
            if c == 'O' {
                links.insert(Point { row, col }, Point { row, col: col + 1 });
                links.insert(Point { row, col: col + 1 }, Point { row, col });
                boxes.insert(Box {
                    position: Point { row, col },
                });
                col += 1;
                boxes.insert(Box {
                    position: Point { row, col },
                });
            }
            if c == '#' {
                walls.insert(Point { row, col });
                col += 1;
                walls.insert(Point { row, col });
            }
            if let Some(direction) = Direction::from_char(c) {
                instructions.push(direction);
            }
            col += 1;
        }
    }
    Problem {
        robot,
        boxes,
        walls,
        instructions,
        links,
    }
}

fn part_one(problem: Problem) -> usize {
    let mut problem = problem.clone();
    problem.render();
    for instruction in problem.instructions.clone().iter() {
        // println!("Moving robot {:?}", instruction);
        let outcome = problem.move_robot(instruction);
        if outcome.box_end_position.is_some() {
            problem.boxes.remove(&Box {
                position: outcome.box_start_position.unwrap(),
            });
            problem.boxes.insert(Box {
                position: outcome.box_end_position.unwrap(),
            });
        }
        problem.robot.position = outcome.robot_position;
        // problem.render();
    }
    let mut score = 0;
    for b in problem.boxes {
        score += b.gps();
    }
    score
}

fn part_two(problem: Problem) -> usize {
    let mut problem = problem.clone();
    problem.render();
    for instruction in problem.instructions.clone().iter() {
        println!("Moving robot {:?}", instruction);
        let outcome = problem.move_robot_part_two(problem.robot.position, instruction);
        if outcome.boxes_to_remove.len() > 0 {
          outcome.boxes_to_remove.iter().for_each(|b| {
            problem.boxes.remove(&b);
            problem.links.remove(&b.position);
          });
          outcome.boxes_to_add.iter().for_each(|b| {
            problem.boxes.insert(b.clone());
          });
          outcome.links_to_add.iter().for_each(|(p1, p2)| {
            problem.links.insert(p1.clone(), p2.clone());
            problem.links.insert(p2.clone(), p1.clone());
          });
        }
        problem.robot.position = outcome.robot_position;
        problem.render();
    }
    let mut score = 0;
    for b in problem.boxes {
        score += b.gps();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_short() {
        let problem = read_input("short.txt");
        assert_eq!(part_one(problem), 2028);
    }

    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(problem), 10092);
    }

    #[test]
    fn test_part_two() {
        let problem = read_input_part_two("test.txt");
        assert_eq!(part_two(problem), 9021);
    }
}

fn main() {
    let problem = read_input("input.txt");
    println!("{}", part_one(problem));
    let problem = read_input_part_two("input.txt");
    println!("{}", part_two(problem));
}
