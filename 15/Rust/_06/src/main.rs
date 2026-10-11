use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(val: &str) -> Point {
        let Some((left, right)) = val.split_once(",") else {
            panic!("Bad Point value")
        };
        let Ok(x) = str::parse(left) else {
            panic!("Bad left value")
        };
        let Ok(y) = str::parse(right) else {
            panic!("Bad right value")
        };
        Point { x, y }
    }
}

#[derive(Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
    Count,
}

#[derive(Clone, Copy)]
struct Instruction {
    action: Action,
    from: Point,
    to: Point,
}

struct Grid {
    low: Point,
    high: Point,
    data: HashMap<Point, bool>,
}

impl Grid {
    fn new(low: Point, high: Point) -> Self {
        Grid {
            low,
            high,
            data: HashMap::new(),
        }
    }

    fn get_mut(&mut self, point: Point) -> Option<&mut bool> {
        self.data.get_mut(&point)
    }

    fn execute(&mut self, instruction: &Instruction) -> usize {
        let from = instruction.from;
        let to = instruction.to;
        let action = instruction.action;
        let mut affected = 0;
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                let point = Point { x, y };
                let has_point = self.data.contains_key(&point);
                if !has_point {
                    self.data.insert(Point { x, y }, false);
                }
                let Some(point) = self.get_mut(point) else {
                    continue;
                };
                match action {
                    Action::Off => {
                        *point = false;
                    }
                    Action::On => {
                        *point = true;
                    }
                    Action::Toggle => {
                        *point = !*point;
                    }
                    Action::Count => {
                        if *point {
                            affected += 1;
                        }
                    }
                }
            }
        }
        affected
    }
}

struct UGrid {
    low: Point,
    high: Point,
    data: HashMap<Point, usize>,
}

impl UGrid {
    fn new(low: Point, high: Point) -> Self {
        UGrid {
            low,
            high,
            data: HashMap::new(),
        }
    }

    fn get_mut(&mut self, point: Point) -> Option<&mut usize> {
        self.data.get_mut(&point)
    }

    fn execute(&mut self, instruction: &Instruction) -> usize {
        let from = instruction.from;
        let to = instruction.to;
        let action = instruction.action;
        let mut affected = 0;
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                let point = Point { x, y };
                let has_point = self.data.contains_key(&point);
                if !has_point {
                    self.data.insert(Point { x, y }, 0);
                }
                let Some(point) = self.get_mut(point) else {
                    continue;
                };
                match action {
                    Action::Off => {
                        if *point == 0 {
                            continue;
                        }
                        *point -= 1;
                    }
                    Action::On => {
                        *point += 1;
                    }
                    Action::Toggle => {
                        *point = *point + 2;
                    }
                    Action::Count => {
                        affected += *point;
                    }
                }
            }
        }
        affected
    }
}

fn read_inputs(filename: &str) -> Vec<Instruction> {
    let bytes = fs::read_to_string(filename).expect("Input file required");
    let lines = bytes.lines();
    lines
        .map(|line| {
            let line = line.replace("turn ", "");
            let words: Vec<&str> = line.split_whitespace().collect();
            let action = match words[0] {
                "toggle" => Action::Toggle,
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!("Bad action string"),
            };
            let from = Point::from(words[1]);
            let to = Point::from(words[3]);
            Instruction { action, from, to }
        })
        .collect()
}

fn part_one(instructions: Vec<Instruction>) -> usize {
    let mut grid = Grid::new(Point { x: 0, y: 0 }, Point { x: 999, y: 999 });
    for instruction in instructions.iter() {
        grid.execute(instruction);
    }
    let from = grid.low.clone();
    let to = grid.high.clone();
    grid.execute(&Instruction {
        from: from,
        to: to,
        action: Action::Count,
    })
}

fn part_two(instructions: Vec<Instruction>) -> usize {
    let mut grid = UGrid::new(Point { x: 0, y: 0 }, Point { x: 999, y: 999 });
    for instruction in instructions.iter() {
        grid.execute(instruction);
    }
    let from = grid.low.clone();
    let to = grid.high.clone();
    grid.execute(&Instruction {
        from: from,
        to: to,
        action: Action::Count,
    })
}

fn main() {
    let inputs = read_inputs("inputs.txt");
    println!("Part one: {}", part_one(inputs.clone()));
    println!("Part two: {}", part_two(inputs));
}
