use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone)]
struct Claim {
    id: String,
    left_pad: usize,
    top_pad: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let id = parts[0].replace("#", "");
        let pads_part = parts[2].replace(":", "");
        let pads: Vec<&str> = pads_part.split(",").collect();
        let left_pad: usize = pads[0].parse().unwrap();
        let top_pad: usize = pads[1].parse().unwrap();
        let dims: Vec<&str> = parts[3].split("x").collect();
        let width: usize = dims[0].parse().unwrap();
        let height: usize = dims[1].parse().unwrap();
        Self {
            id,
            left_pad,
            top_pad,
            width,
            height,
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        let max_row = self.top_pad + self.height;
        let max_col = self.left_pad + self.width;
        let other_max_row = other.top_pad + other.height;
        let other_max_col = other.left_pad + other.width;
        let cols_overlap = (self.left_pad <= other.left_pad && other.left_pad <= max_col)
            || (other.left_pad <= self.left_pad && self.left_pad <= other_max_col)
            || (self.left_pad <= other_max_col && other_max_col <= max_col)
            || (other.left_pad <= max_col && max_col <= other_max_col);
        let rows_overlap = (self.top_pad <= other.top_pad && other.top_pad <= max_row)
            || (other.top_pad <= self.top_pad && self.top_pad <= other_max_row)
            || (self.top_pad <= other_max_row && other_max_row <= max_row)
            || (other.top_pad <= max_row && max_row <= other_max_row);
        cols_overlap && rows_overlap
    }
}

// #1 @ 45,64: 22x22

fn read_input(input_file: &str) -> Vec<Claim> {
    let input = fs::read_to_string(input_file).unwrap();
    input.lines().map(Claim::from_str).collect()
}

fn part_one(claims: Vec<Claim>) -> usize {
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();
    claims.iter().for_each(|claim| {
        let max_row = claim.top_pad + claim.height;
        let max_col = claim.left_pad + claim.width;
        for row in claim.top_pad..max_row {
            for col in claim.left_pad..max_col {
                let key = (row, col);
                if !counts.contains_key(&key) {
                    counts.insert(key, 0);
                }
                let val = counts.get_mut(&key).unwrap();
                *val += 1;
            }
        }
    });

    counts.iter().fold(0, |acc, (_key, val)| {
        if *val > 1 {
            return acc + 1;
        }
        acc
    })
}

fn part_two(claims: Vec<Claim>) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut counts: HashMap<(usize, usize), usize> = HashMap::new();
    claims.iter().for_each(|claim| {
        let max_row = claim.top_pad + claim.height;
        let max_col = claim.left_pad + claim.width;
        for row in claim.top_pad..max_row {
            for col in claim.left_pad..max_col {
                let key = (row, col);
                if !counts.contains_key(&key) {
                    counts.insert(key, 0);
                }
                let val = counts.get_mut(&key).unwrap();
                *val += 1;
            }
        }
    });

    claims.iter().for_each(|claim| {
        if visited.contains(&claim.id) {
            return;
        }
        claims.iter().for_each(|other_claim| {
            if claim.id == other_claim.id {
                return;
            }
            if claim.overlaps(other_claim) {
                visited.insert(claim.id.clone());
                visited.insert(other_claim.id.clone());
            }
        })
    });

    let claims_without_overlaps: Vec<Claim> = claims.into_iter().filter(|claim| !visited.contains(&claim.id)).collect();
    match claims_without_overlaps.len() > 0 {
        true => {
            return Some(claims_without_overlaps[0].id.parse().unwrap());
        },
        false => {
            return None;
        }
    }
}

fn main() {
    let claims = read_input("input.txt");
    println!("Part one: {}", part_one(claims.clone()));
    println!("Part two: {}", part_two(claims).unwrap_or(0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let claims = read_input("test.txt");
        assert_eq!(part_one(claims), 4);
    }

    #[test]
    fn test_overlap() {
        let shape1 = Claim {
            id: "1".to_string(),
            left_pad: 1,
            top_pad: 1,
            width: 3,
            height: 3,
        };

        let shape2 = Claim {
            id: "2".to_string(),
            left_pad: 4,
            top_pad: 4,
            width: 3,
            height: 3,
        };

        assert!(shape1.overlaps(&shape2));

        let shape3 = Claim {
            id: "2".to_string(),
            left_pad: 5,
            top_pad: 5,
            width: 3,
            height: 3,
        };

        assert!(shape2.overlaps(&shape3));
        assert!(!shape1.overlaps(&shape3));

        let shape4 = Claim {
            id: "2".to_string(),
            left_pad: 5,
            top_pad: 2,
            width: 3,
            height: 3,
        };

        assert!(shape3.overlaps(&shape4));
    }

    #[test]
    fn test_part_two() {
        let claims = read_input("test.txt");
        // This test doesn't work for some reason but part_two works on my full input.
        assert_eq!(part_two(claims), Some(3));
    }
}
