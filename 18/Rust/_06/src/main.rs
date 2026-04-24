use std::{
    array::IntoIter,
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
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
    fn iter(&self) -> impl Iterator<Item = Coord> {
        let (lo_row, hi_row) = (self.lower.row, self.upper.row);
        let (lo_col, hi_col) = (self.lower.col, self.upper.col);
        (lo_row..=hi_row)
            .flat_map(move |row| (lo_col..=hi_col).map(move |col| Coord::new(row, col)))
    }

    fn iter_edges(&self) -> impl Iterator<Item = Coord> {
        let (lo_row, hi_row) = (self.lower.row, self.upper.row);
        let (lo_col, hi_col) = (self.lower.col, self.upper.col);
        let left = (lo_row..=hi_row).map(move |row| Coord::new(row, lo_col));
        let right = (lo_row..=hi_row).map(move |row| Coord::new(row, hi_col));
        let top = (lo_col..=hi_col).map(move |col| Coord::new(lo_row, col));
        let bottom = (lo_col..=hi_col).map(move |col| Coord::new(hi_row, col));
        left.chain(right).chain(top).chain(bottom)
    }

    fn expand(&self, by: isize) -> Self {
        BoundingBox {
            lower: Coord::new(self.lower.row - (by / 2), self.lower.col - (by / 2)),
            upper: Coord::new(self.upper.row + (by / 2), self.upper.col + (by / 2)),
        }
    }
}

fn read_input(input_file: &str) -> Vec<Coord> {
    let input = fs::read_to_string(input_file).expect("No input file");
    let coords = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(", ").collect();
            Coord::new(
                parts[0].parse().expect("Bad isize value"),
                parts[1].parse().expect("Bad isize value"),
            )
        })
        .collect();

    coords
}

fn get_bounding_box(input: &Vec<Coord>) -> BoundingBox {
    let mut lower = Coord {
        row: isize::MAX,
        col: isize::MAX,
    };
    let mut upper = Coord {
        row: isize::MIN,
        col: isize::MIN,
    };
    input.iter().for_each(|c| {
        lower.row = std::cmp::min(lower.row, c.row);
        lower.col = std::cmp::min(lower.col, c.col);
        upper.row = std::cmp::max(upper.row, c.row);
        upper.col = std::cmp::max(upper.col, c.col);
    });
    BoundingBox { lower, upper }
}

fn part_one(input: Vec<Coord>) -> usize {
    let mut ownership: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    let bounding_box = get_bounding_box(&input);
    bounding_box.iter().for_each(|c| {
        let mut distance_counts: HashMap<isize, HashSet<Coord>> = HashMap::new();
        input.iter().for_each(|p| {
            let distance = p.manhattan_distance(c);
            match distance_counts.get_mut(&distance) {
                Some(val) => {
                    val.insert(*p);
                }
                None => {
                    distance_counts.insert(distance, HashSet::from([*p]));
                }
            };
        });

        let closest_point = match distance_counts.keys().min() {
            Some(min_distance) => match distance_counts.get(min_distance) {
                Some(fp) if fp.len() == 1 => fp.iter().next(),
                _ => None,
            },
            None => None,
        };

        match closest_point {
            Some(cp) => {
                if !ownership.contains_key(cp) {
                    ownership.insert(*cp, HashSet::new());
                }
                ownership
                    .get_mut(cp)
                    .expect("ownership must exist here")
                    .insert(c);
            }
            None => {}
        }
    });

    bounding_box.iter_edges().for_each(|c| {
        // If any of these edges are in the hashset of an input point, this is an unbounded point and can't be part of the answer.
        ownership.iter_mut().for_each(|(key, val)| {
            if val.contains(&c) {
                val.drain();
            }
        });
    });

    ownership.iter().fold(0, |acc, (key, val)| {
        if val.len() > acc {
            println!("New best from {:?}: {}", key, val.len());
            return val.len();
        }
        return acc;
    })
}

fn part_two(input: Vec<Coord>) -> usize {
    let budget = 10000;
    let bounding_box = get_bounding_box(&input);
    let expanded_box = bounding_box.expand(budget / input.len() as isize);
    expanded_box
        .iter()
        .filter(|c| {
            input
                .iter()
                .map(|p| p.manhattan_distance(*c))
                .sum::<isize>()
                < budget
        })
        .count()
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
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
