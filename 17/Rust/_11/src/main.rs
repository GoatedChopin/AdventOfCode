use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

const DIAGONAL_UNIT_X: f64 = 0.87; // sqrt(3)/2
const DIAGONAL_UNIT_Y: f64 = 0.5; // 1/2

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
            North => HexCoordinate { q: 1, r: -1 },
            NorthEast => HexCoordinate { q: 1, r: 0 },
            SouthEast => HexCoordinate { q: 0, r: 1 },
            South => HexCoordinate { q: -1, r: 1 },
            SouthWest => HexCoordinate { q: -1, r: 0 },
            NorthWest => HexCoordinate { q: 0, r: -1 },
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct HexCoordinate {
    q: isize,
    r: isize,
}

impl HexCoordinate {
    fn s(&self) -> isize {
        0 - self.q - self.r
    }

    fn distance(&self, other: HexCoordinate) -> usize {
        return self.q.abs_diff(other.q) + self.r.abs_diff(other.r);
    }

    fn move_by(&self, dir: HexDir) -> Self {
        let dir_coord: HexCoordinate = dir.into();
        Self {
            q: dir_coord.q + self.q,
            r: dir_coord.r + self.r,
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
    let mut queue = BinaryHeap::from([SearchNode {
        coordinate: start,
        target,
        steps: 0,
    }]);
    let mut visited = HashSet::new();
    let mut max_steps = 0;
    while !queue.is_empty() {
        let Some(current) = queue.pop() else {
            continue;
        };
        // if current.steps > max_steps {
        //     max_steps = current.steps;
        //     println!(
        //         "Visited {} nodes, {} unexplored. max steps is {}",
        //         visited.len(),
        //         queue.len(),
        //         max_steps
        //     )
        // }
        if current.coordinate == current.target {
            return current.steps;
        }
        for dir in HexDir::ALL {
            let new_coord = current.coordinate.move_by(dir);
            if visited.contains(&new_coord) {
                continue;
            }
            visited.insert(new_coord);
            queue.push(SearchNode {
                steps: current.steps + 1,
                coordinate: new_coord,
                target,
            });
        }
    }
    panic!();
}

fn follow_directions(directions: &Vec<HexDir>) -> HexCoordinate {
    let mut current = HexCoordinate { q: 0, r: 0 };
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
    let start = HexCoordinate { q: 0, r: 0 };
    let target = follow_directions(directions);
    println!("{:?} -> {:?}", start, target);
    a_star(start, target)
}

fn main() {
    let directions = read_input("input.txt");
    println!("Part one: {}", part_one(&directions));
}
