use std::collections::{HashMap, HashSet};
use std::fs;

enum ParsingState {
    Prerequisites,
    PrintOrders,
}

fn read_input(path: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut state = ParsingState::Prerequisites;
    let mut prerequisites = HashMap::new();
    let mut print_orders = Vec::new();
    let binding = fs::read_to_string(path).unwrap();
    let lines = binding.lines().collect::<Vec<&str>>();
    for (row, line) in lines.iter().enumerate() {
        if !line.contains("|") {
            state = ParsingState::PrintOrders;
        }
        if line.is_empty() {
            continue;
        }
        match state {
            ParsingState::Prerequisites => {
                let (left, right) = line.split_once("|").unwrap();
                let left = left.trim().parse::<usize>().unwrap();
                let right = right.trim().parse::<usize>().unwrap();
                if !prerequisites.contains_key(&left) {
                    prerequisites.insert(left, Vec::new());
                }
                prerequisites.get_mut(&left).unwrap().push(right);
            }
            ParsingState::PrintOrders => {
                let sequence = line
                    .trim()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                print_orders.push(sequence);
            }
        }
    }
    (prerequisites, print_orders)
}

fn middle_page(updates: &Vec<usize>) -> usize {
    updates[updates.len() / 2]
}

fn is_correct(prerequisites: &HashMap<usize, Vec<usize>>, updates: &Vec<usize>) -> bool {
    for (i, update) in updates.iter().enumerate() {
        for j in 0..i {
            if !prerequisites.contains_key(update) {
                continue;
            }
            let prereqs = prerequisites.get(&update).unwrap();
            if prereqs.contains(&updates[j]) {
                return false;
            }
        }
    }
    true
}

fn part_one(prerequisites: &HashMap<usize, Vec<usize>>, print_orders: &Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    for print_order in print_orders {
        if !is_correct(prerequisites, print_order) {
            continue;
        }
        total += middle_page(print_order);
    }
    total
}

fn get_bad_indexes(prerequisites: &HashMap<usize, Vec<usize>>, print_order: &Vec<usize>) -> Option<(usize, usize)> {
    for (i, update) in print_order.iter().enumerate() {
        for j in 0..i {
            if !prerequisites.contains_key(update) {
                continue;
            }
            let prereqs = prerequisites.get(update).unwrap();
            if prereqs.contains(&print_order[j]) {
                return Some((i, j));
            }
        }
    }
    None
}

fn swap(print_order: &mut Vec<usize>, i: usize, j: usize) {
    let temp = print_order[i];
    print_order[i] = print_order[j];
    print_order[j] = temp;
}

fn correct_print_order(
    prerequisites: &HashMap<usize, Vec<usize>>,
    print_order: &Vec<usize>,
) -> Vec<usize> {
    let mut corrected_print_order = print_order.clone();
    while let Some((i, j)) = get_bad_indexes(prerequisites, &corrected_print_order) {
      swap(&mut corrected_print_order, i, j);
    }
    corrected_print_order
}

fn part_two(prerequisites: &HashMap<usize, Vec<usize>>, print_orders: &Vec<Vec<usize>>) -> usize {
    let bad_print_orders = print_orders
        .iter()
        .filter(|print_order| !is_correct(prerequisites, print_order))
        .collect::<Vec<&Vec<usize>>>();

    let mut total = 0;
    for print_order in bad_print_orders {
        let corrected_print_order = correct_print_order(prerequisites, print_order);
        total += middle_page(&corrected_print_order);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let (prerequisites, print_orders) = read_input("test.txt");
        assert_eq!(part_one(&prerequisites, &print_orders), 143);
    }

    #[test]
    fn test_part_two() {
        let (prerequisites, print_orders) = read_input("test.txt");
        assert_eq!(part_two(&prerequisites, &print_orders), 123);
    }
}

fn main() {
    let (prerequisites, print_orders) = read_input("input.txt");
    println!("Part one: {}", part_one(&prerequisites, &print_orders));
    println!("Part two: {}", part_two(&prerequisites, &print_orders));
}
