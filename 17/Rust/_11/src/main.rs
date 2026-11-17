use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fs;

const DIAGONAL_UNIT: isize = 7; // 1/2

#[derive(Clone, Copy)]
enum HexDir {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl HexDir {
    const ALL: [HexDir; 6] = [
        HexDir::North,
        HexDir::NorthEast,
        HexDir::SouthEast,
        HexDir::South,
        HexDir::SouthWest,
        HexDir::NorthWest,
    ];
}

impl TryFrom<&str> for HexDir {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<HexDir, Self::Error> {
        use HexDir::*;
        match s {
            "n" => Ok(North),
            "ne" => Ok(NorthEast),
            "se" => Ok(SouthEast),
            "s" => Ok(South),
            "sw" => Ok(SouthWest),
            "nw" => Ok(NorthWest),
            _ => Err("Bad input string for HexDir, supported values are {n,ne,se,s,sw,nw,}"),
        }
    }
}

impl Into<HexCoordinate> for HexDir {
    fn into(self) -> HexCoordinate {
        use HexDir::*;
        match self {
            North => HexCoordinate { x: 0, y: 10 },
            NorthEast => HexCoordinate {
                x: DIAGONAL_UNIT,
                y: DIAGONAL_UNIT,
            },
            SouthEast => HexCoordinate {
                x: DIAGONAL_UNIT,
                y: -DIAGONAL_UNIT,
            },
            South => HexCoordinate { x: 0, y: -10 },
            SouthWest => HexCoordinate {
                x: -DIAGONAL_UNIT,
                y: -DIAGONAL_UNIT,
            },
            NorthWest => HexCoordinate {
                x: -DIAGONAL_UNIT,
                y: DIAGONAL_UNIT,
            },
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct HexCoordinate {
    x: isize,
    y: isize,
}

impl HexCoordinate {
    fn distance(&self, other: HexCoordinate) -> usize {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
    }

    fn move_by(&self, dir: HexDir) -> Self {
        let dir_coord: HexCoordinate = dir.into();
        Self {
            x: dir_coord.x + self.x,
            y: dir_coord.x + self.y,
        }
    }
}

struct SearchNode {
    steps: usize,
    coordinate: HexCoordinate,
    target: HexCoordinate,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl Eq for SearchNode {}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            (self.steps + self.coordinate.distance(self.target))
                .cmp(&(other.steps + other.coordinate.distance(other.target))),
        )
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.steps + self.coordinate.distance(self.target))
            .cmp(&(other.steps + other.coordinate.distance(other.target)))
    }
}

fn a_star(start: HexCoordinate, target: HexCoordinate) -> usize {
    let mut queue = BinaryHeap::from([Reverse(SearchNode {
        coordinate: start,
        target,
        steps: 0,
    })]);
    let mut visited = HashSet::new();
    let mut max_steps = 0;
    while !queue.is_empty() {
        let Some(Reverse(current)) = queue.pop() else {
            continue;
        };
        if current.steps > max_steps {
            max_steps = current.steps;
            println!(
                "Visited {} nodes, {} unexplored. max steps is {}",
                visited.len(),
                queue.len(),
                max_steps
            )
        }
        if current.coordinate == current.target {
            return current.steps;
        }
        for dir in HexDir::ALL {
            let new_coord = current.coordinate.move_by(dir);
            if visited.contains(&new_coord) {
                continue;
            }
            visited.insert(new_coord);
            queue.push(Reverse(SearchNode {
                steps: current.steps + 1,
                coordinate: new_coord,
                target,
            }));
        }
    }
    panic!();
}

fn follow_directions(directions: &Vec<HexDir>) -> HexCoordinate {
    let mut current = HexCoordinate { x: 0, y: 0 };
    for direction in directions {
        current = current.move_by(*direction);
    }
    current
}

fn read_input(path: &str) -> Vec<HexDir> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.trim()
        .split(",")
        .map(|d| d.try_into().expect("Bad direction value"))
        .collect()
}

fn part_one(directions: &Vec<HexDir>) -> usize {
    let start = HexCoordinate { x: 0, y: 0 };
    let target = follow_directions(directions);
    println!("{:?} -> {:?}", start, target);
    a_star(start, target)
}

fn main() {
    let directions = read_input("input.txt");
    println!("Part one: {}", part_one(&directions));
}
