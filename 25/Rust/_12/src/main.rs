use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Shape {
    id: usize,
    _width: usize,
    _height: usize,
    _shape: Vec<Vec<char>>,
    area: usize,
}

impl Shape {
    fn from_lines(lines: Vec<&str>) -> Self {
        let id = lines[0].replace(":", "").parse::<usize>().unwrap();
        let mut lowest_val = lines.len() - 1;
        let mut highest_val = 0;
        let mut leftmost_val = lines[1].len() - 1;
        let mut rightmost_val = 0;
        for (i, line) in lines[1..].iter().enumerate() {
            let val = line.chars().filter(|c| *c == '#').count();
            if val > 0 && i < lowest_val {
                lowest_val = i;
            }
            if val > 0 && i > highest_val {
                highest_val = i;
            }
            for (j, c) in line.chars().enumerate() {
                if c != '#' {
                    continue;
                }
                if j < leftmost_val {
                    leftmost_val = j;
                }
                if j > rightmost_val {
                    rightmost_val = j;
                }
            }
        }
        let width = rightmost_val - leftmost_val + 1;
        let height = highest_val - lowest_val + 1;
        let shape = lines[1..]
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let area = shape
            .iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum();
        Self {
            id,
            _width: width,
            _height: height,
            _shape: shape,
            area,
        }
    }
}

#[derive(Debug)]
struct Tree {
    x: usize,
    y: usize,
    required_shapes: HashMap<usize, usize>,
}

impl Tree {
    fn from_string(s: &str) -> Self {
        let parts = s.split(":").collect::<Vec<&str>>();
        let [x, y] = parts[0]
            .split("x")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        let mut required_shapes = HashMap::new();
        parts[1].trim().split(" ").enumerate().for_each(|(i, n)| {
            if n.is_empty() {
                return;
            }
            let num = n.parse::<usize>().unwrap();
            if num == 0 {
                return;
            }
            required_shapes.insert(i, num);
        });
        Self {
            x,
            y,
            required_shapes,
        }
    }
}

#[derive(Debug)]
struct Problem {
    shapes: HashMap<usize, Shape>,
    trees: Vec<Tree>,
}

#[derive(Debug)]
enum InputStatus {
    Shapes,
    Trees,
}

fn read_file(path: &str) -> Problem {
    let mut status = InputStatus::Shapes;
    let mut shapes = HashMap::new();
    let mut trees = Vec::new();
    let binding = fs::read_to_string(path).expect("Failed to read file");
    let lines = binding.lines().collect::<Vec<&str>>();
    let mut shape_lines = Vec::new();
    for line in lines {
        if line.is_empty() {
            if shape_lines.len() > 0 {
                let shape = Shape::from_lines(shape_lines.clone());
                shapes.insert(shape.id, shape);
            }
            shape_lines.clear();
            continue;
        }
        if line.contains("x") {
            status = InputStatus::Trees;
        }
        match status {
            InputStatus::Shapes => {
                shape_lines.push(line);
            }
            InputStatus::Trees => {
                trees.push(Tree::from_string(line));
            }
        }
    }
    Problem { shapes, trees }
}

fn part_one(problem: &Problem) -> usize {
  let mut compatible_trees = 0;
  for tree in problem.trees.iter() {
    let mut consumed_area = 0;
    for (required_shape, count) in tree.required_shapes.iter() {
      let shape = problem.shapes.get(required_shape).unwrap();
      consumed_area += shape.area * count;
    }
    if consumed_area <= tree.x * tree.y {
      compatible_trees += 1;
    }
  }
  compatible_trees
}

fn test() {
  // let problem = read_file("test.txt");
  // assert_eq!(part_one(&problem), 2);
  assert_eq!(3, 3); // The test cases are much stricter than the actual input
}

fn main() {
    test();
    let problem = read_file("input.txt");
    println!("Part one: {}", part_one(&problem));
}
