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

fn is_balanced(
    input: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, usize>,
    key: &String,
    depth: usize,
) -> bool {
    let children = input.get(key).unwrap();
    let mut child_weights = HashMap::new();
    for child in children.clone() {
        let new_weight = weights.get(&child).unwrap();
        let child_weight = get_weight(input, weights, &child, *new_weight, depth + 1);
        child_weights.insert(child, child_weight);
    }
    for (child1, weight) in child_weights.clone() {
        for (child2, weight2) in child_weights.clone() {
            if child1 == child2 {
                continue;
            }
            if weight != weight2 {
                // println!(
                //     "Unbalanced Child: {} -> {} vs {} -> {}",
                //     child1, weight, child2, weight2
                // );
                return false;
            }
        }
    }
    return true;
}

struct QueueItem {
    node: String,
    depth: usize,
    parent_is_balanced: bool,
}

fn get_nodes_at_depth(
    input: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, usize>,
    key: &String,
    depth: usize,
    current_depth: usize,
) -> Vec<String> {
    let mut nodes = Vec::new();
    let children = input.get(key).unwrap();
    for child in children.clone() {
        if current_depth == depth {
            nodes.push(child.clone());
        }
        nodes.extend(get_nodes_at_depth(
            input,
            weights,
            &child,
            depth,
            current_depth + 1,
        ));
    }
    // for node in nodes.clone() {
    //     println!(
    //         "{} Node: {} -> {} + {} = {}",
    //         depth,
    //         node,
    //         weights.get(&node).unwrap(),
    //         get_weight(input, weights, &node, 0, depth),
    //         get_weight(input, weights, &node, 0, depth) + weights.get(&node).unwrap()
    //     );
    // }
    nodes
}
fn part_two(
    input: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, usize>,
    root: String,
) -> usize {
    let mut depth = 0;
    // let mut unbalanced_node: String = "".to_string();
    // let mut unbalanced_weight: usize = 0;
    let mut current_node = root.clone();
    let mut queue = VecDeque::new();
    // let mut depth_balance_map = HashMap::new();
    let mut parent_is_balanced = false;
    let mut children_are_balanced: bool;
    queue.push_back(QueueItem {
        node: current_node.clone(),
        depth: depth,
        parent_is_balanced: parent_is_balanced,
    });
    while queue.len() > 0 {
        let item = queue.pop_front().unwrap(); // Needs to pop_back or pop_left so we are doing BFS
        current_node = item.node;
        depth = item.depth;
        parent_is_balanced = item.parent_is_balanced;
        children_are_balanced = true;
        let children = input.get(&current_node).unwrap();
        for child in children.clone() {
            children_are_balanced =
                children_are_balanced && is_balanced(input, weights, &child, depth + 1);
            queue.push_back(QueueItem {
                node: child.clone(),
                depth: depth + 1,
                parent_is_balanced: is_balanced(input, weights, &current_node, depth + 1),
            });
        }
        if !parent_is_balanced {
          println!(
              "Current node: {}, Parent is balanced: {}, Children are balanced: {} - {}",
              current_node, parent_is_balanced, children_are_balanced, depth
          );
        }
        if !parent_is_balanced && children_are_balanced {
            // println!("Found unbalanced node: {}", current_node);
            let nodes = get_nodes_at_depth(input, weights, &current_node, depth, 0);
            let mut self_weights = HashMap::new();
            let mut child_weights = HashMap::new();
            let mut votes = HashMap::new();
            let mut expected_weight = 0;
            for node in nodes.clone() {
                let weight = weights.get(&node).unwrap();
                let child_weight = get_weight(input, weights, &node, 0, depth);
                // vote on what the weight of each child node should be
                self_weights.entry(node.clone()).or_insert(weight);
                child_weights.entry(node.clone()).or_insert(child_weight);
                votes.entry(weight + child_weight).or_insert(0);
                votes.insert(
                    weight + child_weight,
                    votes.get(&(*weight + child_weight)).unwrap() + 1,
                );
                if *votes.get(&(weight + child_weight)).unwrap() > 1 {
                    expected_weight = weight + child_weight;
                }
            }
            for node in nodes.clone() {
                let weight = self_weights.get(&node).unwrap();
                let child_weight = child_weights.get(&node).unwrap();
                if *votes.get(&(*weight + child_weight)).unwrap() > 1 {
                    continue;
                }
                // adjust the weight of the current node to the expected weight
                let new_weight = expected_weight - child_weight;
                return new_weight;
            }
        }
    }
    println!("No unbalanced node found");
    return 0;
}

fn get_weight(
    input: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, usize>,
    key: &String,
    starting_weight: usize,
    depth: usize,
) -> usize {
    let mut total_weight = starting_weight;
    let children = input.get(key).unwrap();
    for child in children.clone() {
        let new_weight = weights.get(&child).unwrap();
        let child_weight = get_weight(input, weights, &child, *new_weight, depth + 1);
        total_weight += child_weight;
    }
    return total_weight;
}

fn main() {
    test();
    let data = read_input("input.txt");
    let root = part_one(&data.connections);
    println!("Part one: {}", root.clone());
    println!(
        "Part two: {}",
        part_two(&data.connections, &data.weights, root)
    );
}

fn test() {
    println!("Testing...");
    let input = read_input("test.txt");

    panic!("Not working yet");
    let root = part_one(&input.connections);
    assert_eq!(root.clone(), "tknk");
    assert_eq!(part_two(&input.connections, &input.weights, root), 60);
    println!("Test passed");
}
