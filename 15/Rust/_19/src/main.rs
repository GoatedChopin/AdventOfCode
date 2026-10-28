use std::collections::HashSet;
use std::fs;

struct Replacement {
    input: String,
    out: String,
}

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
            let (input, out) = line.split_once(" => ").expect("Bad rule input");
            Replacement {
                input: input.to_string(),
                out: out.to_string(),
            }
        })
        .collect();
    Molecule {
        rules,
        input: input.to_string(),
    }
}

fn distinct_replacements(input: String, rules: Vec<Replacement>) -> HashSet<String> {
    let mut distinct = HashSet::new();
    rules.iter().for_each(|rule| {
        distinct.insert(input.clone().replacen(rule.input, rule.out, 1));
    });
    distinct
}

fn part_one(molecule: Molecule) -> usize {}

fn main() {
    let molecule = read_input("input.txt");
    println!("Part one: {}", part_one(molecule));
}
