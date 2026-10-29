use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone)]
struct Replacement {
    input: String,
    out: String,
}

#[derive(Clone)]
struct Molecule {
    rules: Vec<Replacement>,
    input: String,
}

fn read_input(path: &str) -> Molecule {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let (rules_raw, input) = s.split_once("\n\n").expect("Input must have two parts");
    let rules = rules_raw
        .lines()
        .map(|line| {
            let (input, out) = line.trim().split_once(" => ").expect("Bad rule input");
            Replacement {
                input: input.to_string(),
                out: out.to_string(),
            }
        })
        .collect();
    Molecule {
        rules,
        input: input.trim().to_string(),
    }
}

fn distinct_replacements(input: String, rules: &Vec<Replacement>) -> HashSet<String> {
    let mut distinct = HashSet::new();
    rules.iter().for_each(|rule| {
        for l in 0..=input.len() {
            let Some(slice) = input.get(l..(l + rule.input.len())) else {
                continue;
            };
            if slice == rule.input {
                let (left, right_raw) = input.split_at(l);
                let (_old, right) = right_raw.split_at(rule.input.len());
                let new_string: String = left.to_string() + &rule.out + right;
                distinct.insert(new_string);
            }
        }
    });
    distinct
}

fn part_one(molecule: Molecule) -> usize {
    let dr = distinct_replacements(molecule.input, &molecule.rules);
    println!("{:?}", dr);
    dr.len()
}

fn part_two(molecule: Molecule) -> usize {
    let mut queue = VecDeque::from([(0, "e".to_string())]);
    let mut visited = HashSet::from(["e".to_string()]);
    while !queue.is_empty() {
        let Some((steps, current_string)) = queue.pop_front() else {
            break;
        };
        if current_string == molecule.input {
            return steps;
        }
        let new_strings = distinct_replacements(current_string, &molecule.rules);
        for s in new_strings.iter() {
            if visited.contains(s) {
                continue;
            }
            queue.push_back((steps + 1, s.clone()));
        }
        visited.extend(new_strings);
    }
    panic!("No solution found")
}

fn main() {
    let molecule = read_input("input.txt");
    println!("Part one: {}", part_one(molecule.clone()));
    println!("Part two: {}", part_two(molecule));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let molecule = read_input("test.txt");
        assert_eq!(part_one(molecule), 4);
    }
}
