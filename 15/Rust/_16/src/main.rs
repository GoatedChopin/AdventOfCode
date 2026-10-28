use std::{collections::HashMap, fs};

use crate::Operation::GreaterThan;

#[derive(Debug, Clone)]
struct Sue(HashMap<String, usize>);

enum Operation {
    EqualTo,
    GreaterThan,
    LessThan,
}

fn read_input(path: &str) -> Vec<Sue> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.lines()
        .map(|sue| {
            let mut map: HashMap<String, usize> = HashMap::new();
            let props = sue.split_once(": ").expect("Bad sue line").1.split(",");
            props.into_iter().for_each(|prop| {
                let Some((key, value)) = prop.split_once(": ") else {
                    return;
                };
                map.insert(
                    key.trim().to_string(),
                    value.trim().parse().expect("Bad integer value"),
                );
            });
            Sue(map)
        })
        .collect()
}

fn part_one(my_sue: Sue, sues: Vec<Sue>) -> Option<usize> {
    sues.iter()
        .enumerate()
        .find(|(i, s)| {
            s.0.iter().all(|(key, val)| match my_sue.0.get(key) {
                Some(v) => v == val,
                None => true,
            })
        })
        .map(|(i, _)| i + 1)
}

fn operation(s: &String) -> Operation {
    let greater_than = |s: &String| *s == "cats".to_string() || *s == "trees".to_string();
    let less_than = |s: &String| *s == "pomeranians".to_string() || *s == "goldfish".to_string();

    match (greater_than(s), less_than(s)) {
        (true, _) => Operation::GreaterThan,
        (_, true) => Operation::LessThan,
        _ => Operation::EqualTo,
    }
}

fn part_two(my_sue: Sue, sues: Vec<Sue>) -> Option<usize> {
    sues.iter()
        .enumerate()
        .find(|(i, s)| {
            s.0.iter().all(|(key, val)| match operation(key) {
                Operation::GreaterThan => match my_sue.0.get(key) {
                    Some(v) => v < val,
                    None => true,
                },
                Operation::LessThan => match my_sue.0.get(key) {
                    Some(v) => v > val,
                    None => true,
                },
                _ => match my_sue.0.get(key) {
                    Some(v) => v == val,
                    None => true,
                },
            })
        })
        .map(|(i, _)| i + 1)
}

fn main() {
    let sues = read_input("input.txt");
    let my_sue = read_input("sue.txt").pop().expect("My sue must exist");
    println!("Part one: {:?}", part_one(my_sue.clone(), sues.clone()));
    println!("Part two: {:?}", part_two(my_sue, sues));
}
