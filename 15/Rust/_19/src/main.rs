use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug)]
struct Replacement {
    input: String,
    out: String,
}

impl Replacement {
    fn inverse(&self) -> Self {
        Self {
            input: self.out.clone(),
            out: self.input.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct Molecule {
    rules: Vec<Replacement>,
    input: String,
}

impl Molecule {
    fn inverse(&self) -> Self {
        Self {
            input: self.input.clone(),
            rules: self.rules.iter().map(|r| r.inverse()).collect(),
        }
    }
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

fn apply_mutations(input: String, mutations: Vec<(usize, String, String)>) -> String {
    let mut new_string = String::new();
    let mut left = 0;
    for (i, before, after) in mutations.clone() {
        if i > left {
            new_string += &input.get(left..i).unwrap_or(&"".to_string());
        }
        left = i + before.len();
        new_string += &after;
    }
    println!("{} -> {:?} -> {}", input, mutations, new_string);
    new_string
}

fn generate(
    pairs: &[(usize, String, String)],
    index: usize,
    current: &mut Vec<(usize, String, String)>,
    results: &mut Vec<Vec<(usize, String, String)>>,
) {
    // Base case, nothing to apply
    if index >= pairs.len() {
        results.push(current.clone());
        return;
    }
    // Case 1, the current mutation does not overlap
    if let Some((i, before, after)) = pairs.get(pairs.len() - 1) {
        match current.get(current.len().saturating_sub(1)) {
            Some((li, lbefore, lafter)) => {}
            None => current.push((i.clone(), before.clone(), after.clone())),
        }
    }
}

fn multi_replacements(input: String, rules: &Vec<Replacement>) -> HashSet<(usize, String)> {
    let mut distinct = HashSet::new();
    let mut pairs = Vec::new();
    for l in 0..=input.len() {
        rules.iter().for_each(|rule| {
            let Some(slice) = input.get(l..(l + rule.input.len())) else {
                return;
            };
            if slice == rule.input {
                pairs.push((l, rule.input.clone(), rule.out.clone()));
            }
        });
    }
    pairs.sort();
    println!("{:?}", pairs);

    let mut mutations = Vec::from([Vec::new()]);

    for (i, before, after) in pairs.into_iter() {
        for m in mutations.iter_mut() {
            let last_index = m.get(m.len().saturating_sub(1));
            match last_index {
                Some((li, _lbefore, _lafter)) => {
                    if li + before.len() <= i {
                        // The mutations don't overlap
                        m.push((i, before.clone(), after.clone()));
                        continue;
                    }
                    let mut new_mutations = m.clone();
                    new_mutations.pop();
                    new_mutations.push((i, before.clone(), after.clone()));
                }
                None => m.push((i, before.clone(), after.clone())),
            }
        }
    }

    mutations.into_iter().for_each(|m| {
        distinct.insert((m.len(), apply_mutations(input.clone(), m)));
    });

    // a -> bb
    // c -> b
    // bbbb
    // (0, bb, a) (0, b, c) (1, bb, a) (1, b, c) (2, bb, a) (2, b, c) (3, b, c)
    //
    // abb
    distinct
}

fn part_one(molecule: Molecule) -> usize {
    let dr = distinct_replacements(molecule.input, &molecule.rules);
    println!("{:?}", dr);
    dr.len()
}

fn part_two(molecule: Molecule) -> usize {
    let inverse = molecule.inverse();
    let mut queue = VecDeque::from([(0, inverse.input)]);
    let mut visited = HashSet::from(["e".to_string()]);
    let e = "e".to_string();
    while !queue.is_empty() {
        let Some((steps, current_string)) = queue.pop_front() else {
            break;
        };
        if current_string == e {
            return steps;
        }
        let new_strings = multi_replacements(current_string, &inverse.rules);
        for (new_steps, s) in new_strings.iter() {
            if visited.contains(s) {
                continue;
            }
            println!("{} -> {}", s, steps + new_steps);
            queue.push_back((steps + new_steps, s.clone()));
            visited.insert(s.clone());
        }
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

    #[test]
    fn test_part_two() {
        let molecule = read_input("test2.txt");
        println!("{:?}", molecule.inverse());
        assert_eq!(part_two(molecule), 6);
    }
}
