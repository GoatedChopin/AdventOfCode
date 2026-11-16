use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

struct Data {
    connections: HashMap<String, Vec<String>>,
    weights: HashMap<String, usize>,
}

fn read_input(path: &str) -> Data {
    let mut map = HashMap::new();
    let mut weights = HashMap::new();

    for line in fs::read_to_string(path).unwrap().trim().to_string().lines() {
        let parts = line
            .replace(",", "")
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let key = parts[0].clone();
        let value: Vec<String>;
        if parts.len() == 2 {
            value = Vec::new();
        } else {
            value = parts[3..].to_vec();
        }
        map.insert(key.clone(), value.clone());
        weights.insert(
            key.clone(),
            parts[1]
                .replace("(", "")
                .replace(")", "")
                .parse::<usize>()
                .unwrap(),
        );
    }
    Data {
        connections: map,
        weights: weights,
    }
}

fn visit(input: &HashMap<String, Vec<String>>, visited: &mut HashMap<String, usize>, key: &String) {
    if !visited.contains_key(key) {
        visited.insert(key.clone(), 0);
    }
    *visited.get_mut(key).unwrap() += 1;
    for value in input.get(key).unwrap().clone() {
        visit(input, visited, &value);
    }
}

fn part_one(input: &HashMap<String, Vec<String>>) -> String {
    // find the "root" of the tree
    let mut visited = HashMap::new();
    for (key, _) in input {
        visit(input, &mut visited, key);
    }

    for (key, value) in visited {
        if value == 1 && !input.get(&key).unwrap().is_empty() {
            return key.clone();
        }
    }
    return "Failed to find root".to_string();
}

// Traverse the tree
// Each child node has a weight which is the sum of the weights of its children and itself
// For each node, calculate its weight and the weights of its children
// If each of a node's children have the same weight, then the node is balanced
// Only one node is unbalanced, and we need to find the weight that needs to be adjusted to balance the tree

// Algorithm:
// Find the node where its parent is unbalanced and its children are balanced
// 1. Traverse the tree
// 2. For each node, track whether the path so far is balanced or unbalanced using a boolean flag
// 3. If the parent is unbalanced and the children are balanced, return the current node's weight and depth

fn weight(s: &String, data: &Data) -> usize {
    let mut out = 0;
    if let Some(w) = data.weights.get(s) {
        out += *w;
    }
    if let Some(children) = data.connections.get(s) {
        out += children.iter().map(|c| weight(c, data)).sum::<usize>();
    }
    out
}

fn is_balanced(start: &String, data: &Data) -> bool {
    match data.connections.get(start) {
        Some(children) => {
            let weights: Vec<usize> = children.iter().map(|c| weight(c, data)).collect();
            if weights.is_empty() {
                return true;
            }
            let w = weights[0];
            weights.into_iter().all(|c| c == w)
        }
        None => true,
    }
}

fn part_two(data: Data) -> usize {
    let root = part_one(&data.connections.clone());
    let mut queue = VecDeque::from([root]);
    while !queue.is_empty() {
        let Some(current) = queue.pop_front() else {
            continue;
        };
        if is_balanced(&current, &data) {
            continue;
        }
        let Some(children) = data.connections.get(&current) else {
            continue;
        };
        children
            .iter()
            .for_each(|child| queue.push_back(child.clone()));
        let children_balanced: Vec<bool> = children.iter().map(|c| is_balanced(c, &data)).collect();
        if children_balanced.iter().all(|c| *c) {
            // We found the right part, all children are balanced in their subtrees, but the parent is not. One must be adjusted.
            let sums: Vec<usize> = children.iter().map(|c| weight(c, &data)).collect();
            let distinct_sums: HashMap<usize, usize> =
                sums.iter().fold(HashMap::new(), |mut acc, sum| {
                    match acc.get_mut(sum) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            acc.insert(*sum, 1);
                        }
                    }
                    return acc;
                });
            let Some(bad_sum) = distinct_sums
                .iter()
                .filter_map(|(sum, count)| if *count == 1 { Some(*sum) } else { None })
                .collect::<Vec<usize>>()
                .pop()
            else {
                continue;
            };
            let Some(good_sum) = distinct_sums
                .iter()
                .filter_map(|(sum, count)| if *count > 1 { Some(*sum) } else { None })
                .collect::<Vec<usize>>()
                .pop()
            else {
                continue;
            };
            for (sum, child) in sums.iter().zip(children.iter()) {
                if *sum == good_sum {
                    continue;
                }
                let Some(bad_weight) = data.weights.get(child) else {
                    continue;
                };
                println!("Vars {}, {}, {}", good_sum, bad_sum, bad_weight);
                match good_sum > bad_sum {
                    true => return (good_sum - bad_sum) + bad_weight,
                    false => return bad_weight - (bad_sum - good_sum),
                }
            }
        }
    }
    panic!();
}

fn main() {
    let data = read_input("input.txt");
    let root = part_one(&data.connections);
    println!("Part one: {}", root.clone());
    println!("Part two: {}", part_two(data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_both_parts() {
        let input = read_input("test.txt");
        let root = part_one(&input.connections);
        assert_eq!(root.clone(), "tknk");
        assert_eq!(part_two(input), 60);
    }
}
