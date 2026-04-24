use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    row: isize,
    col: isize,
    unbounded: bool,
}

impl Coord {
    fn new(row: isize, col: isize) -> Self {
        Self {
            row,
            col,
            unbounded: false,
        }
    }

    fn manhattan_distance(&self, other: Coord) -> isize {
        let row_diff = match self.row > other.row {
            true => self.row - other.row,
            false => other.row - self.row,
        };
        let col_diff = match self.col > other.col {
            true => self.col - other.col,
            false => other.col - self.col,
        };
        return row_diff + col_diff;
    }

    fn move_by(&self, d: (isize, isize)) -> Self {
        Self::new(self.row + d.0, self.col + d.1)
    }

    fn get_neighbors(&self) -> Vec<Self> {
        CARDINAL_DIRECTIONS
            .iter()
            .map(|d| self.move_by(*d))
            .collect()
    }
}

struct BoundingBox {
    lower: Coord,
    upper: Coord,
}

impl BoundingBox {
    fn contains(&self, c: &Coord) -> bool {
        self.lower.row <= c.row
            && self.lower.col <= c.col
            && self.upper.row >= c.row
            && self.upper.col >= c.col
    }
}

fn read_input(input_file: &str) -> Vec<Coord> {
    let input = fs::read_to_string(input_file).expect("No input file");
    let coords = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(", ").collect();
            Coord {
                row: parts[0].parse().expect("Bad isize value"),
                col: parts[1].parse().expect("Bad isize value"),
                unbounded: false,
            }
        })
        .collect();

    coords
}

fn get_bounding_box(input: &Vec<Coord>) -> BoundingBox {
    let mut lower = Coord {
        row: isize::MAX,
        col: isize::MAX,
        unbounded: false,
    };
    let mut upper = Coord {
        row: isize::MIN,
        col: isize::MIN,
        unbounded: false,
    };
    input.iter().for_each(|c| {
        lower.row = std::cmp::min(lower.row, c.row);
        lower.col = std::cmp::min(lower.col, c.col);
        upper.row = std::cmp::max(upper.row, c.row);
        upper.col = std::cmp::max(upper.col, c.col);
    });
    BoundingBox { lower, upper }
}

fn flood_fill(
    c: &mut Coord,
    bounding_box: &BoundingBox,
    ownership: &mut HashMap<Coord, Coord>,
    shortest_paths: &mut HashMap<Coord, isize>,
) {
    let mut queue = VecDeque::from([(*c, 0)]);
    let mut visited = HashSet::from([*c]);
    ownership.insert(*c, *c);
    shortest_paths.insert(*c, 0);
    while queue.len() > 0 {
        let (current, steps) = queue.pop_back().expect("Strange queue behavior");
        let neighbors = current.get_neighbors();
        for neighbor in neighbors.into_iter() {
            if !bounding_box.contains(&neighbor) {
                c.unbounded = true;
                continue;
            }
            if visited.contains(&neighbor) {
                continue;
            }
            visited.insert(neighbor);
            let prev_best = shortest_paths.get(&neighbor);
            match prev_best {
                Some(prev_steps) => {
                    if *prev_steps == steps {
                        ownership.remove(&neighbor);
                    } else if *prev_steps < steps {
                        continue;
                    } else if *prev_steps > steps {
                        ownership.insert(neighbor, *c);
                        shortest_paths.insert(neighbor, steps);
                    }
                }
                None => {
                    shortest_paths.insert(neighbor, steps);
                    ownership.insert(neighbor, *c);
                }
            }
            queue.push_front((neighbor, steps + 1));
        }
    }
}

fn part_one(input: Vec<Coord>) -> usize {
    let mut input = input;
    let bounding_box = get_bounding_box(&input);
    let mut ownership: HashMap<Coord, Coord> = HashMap::new();
    let mut reachable: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    let mut shortest_paths: HashMap<Coord, isize> = HashMap::new();

    input.iter_mut().for_each(|c| {
        flood_fill(c, &bounding_box, &mut ownership, &mut shortest_paths);
    });

    ownership.iter().for_each(|(n, c)| {
        let c_reachable = reachable.get_mut(c);
        match c_reachable {
            Some(r) => {
                r.insert(*n);
            }
            None => {
                reachable.insert(*c, HashSet::from([*n]));
            }
        }
    });

    let max_reachable = reachable
        .iter()
        .fold(Coord::new(0, 0), |acc, (coord, c_reachable)| {
            if coord.unbounded {
                return acc;
            }
            let current_best = reachable.get(&acc);
            let cb_reachable = match current_best {
                Some(cb) => cb.len(),
                None => 0,
            };
            match cb_reachable > c_reachable.len() {
                true => acc,
                false => *coord,
            }
        });

    println!("Best is {:?}", max_reachable);

    reachable
        .get(&max_reachable)
        .unwrap_or(&HashSet::new())
        .len()
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 17);
    }
}
