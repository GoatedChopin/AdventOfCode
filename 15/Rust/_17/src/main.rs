use std::collections::HashSet;
use std::fs;

fn config(visited: HashSet<usize>) -> Vec<usize> {
    let mut v: Vec<usize> = visited.into_iter().collect();
    v.sort();
    v
}

fn combinations(
    amount: usize,
    visited: HashSet<usize>,
    containers: &Vec<usize>,
) -> HashSet<Vec<usize>> {
    if amount == 0 {
        return HashSet::from([config(visited)]);
    }
    let mut c = HashSet::new();
    for (i, container) in containers.iter().enumerate() {
        if visited.contains(&i) {
            continue;
        }
        if amount < *container {
            continue;
        }
        let mut new_visited = visited.clone();
        new_visited.insert(i);
        c.extend(combinations(amount - *container, new_visited, containers));
    }
    c
}

fn read_input(path: &str) -> Vec<usize> {
    let i = fs::read_to_string(path).expect("Input file must exist");
    i.lines()
        .into_iter()
        .map(|l| l.parse().expect("Bad input usize"))
        .collect()
}

fn part_one(amount: usize, containers: &Vec<usize>) -> usize {
    let visited = HashSet::new();
    combinations(amount, visited, containers).len()
}

fn part_two(amount: usize, containers: &Vec<usize>) -> usize {
    let visited = HashSet::new();
    let c = combinations(amount, visited, containers);
    let Some(min_containers) = c
        .iter()
        .min_by(|x, y| x.len().cmp(&y.len()))
        .map(|container| container.len())
    else {
        return 0;
    };
    c.iter()
        .filter(|container| container.len() == min_containers)
        .count()
}

fn main() {
    let amount = 150;
    let containers = read_input("input.txt");
    println!("Part one: {}", part_one(amount, &containers));
    println!("Part two: {}", part_two(amount, &containers));
}
