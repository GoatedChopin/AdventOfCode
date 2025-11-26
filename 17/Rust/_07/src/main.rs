use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

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
        let child_weight = get_weight(
            input,
            weights,
            &child,
            *new_weight,
            depth + 1,
        );
        child_weights.insert(child, child_weight);
    }
    for (child1, weight) in child_weights.clone() {
        for (child2, weight2) in child_weights.clone() {
            if child1 != child2 && weight != weight2 {
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

fn part_two(input: &HashMap<String, Vec<String>>, weights: &HashMap<String, usize>, root: String) -> usize {
    let mut depth = 0;
    let mut unbalanced_node: String = "".to_string();
    let mut unbalanced_weight: usize = 0;
    let mut current_node = root.clone();

    let mut queue = Vec::new();

    let mut parent_is_balanced = false;
    let mut children_are_balanced = true;
    queue.push(QueueItem {
      node: current_node.clone(),
      depth: depth,
      parent_is_balanced: parent_is_balanced,
    });
    while !(!parent_is_balanced && children_are_balanced) {
      let item = queue.pop().unwrap();
      current_node = item.node;
      depth = item.depth + 1;
      parent_is_balanced = item.parent_is_balanced;
      let children = input.get(&current_node).unwrap();
      let mut child_weights = HashMap::new();
      for child in children.clone() {
        let child_weight = get_weight(
          input,
          weights,
          &child,
          0,
          depth + 1,
        );
        child_weights.insert(child, child_weight);
      }
      children_are_balanced = true;
      let mut unique_weights = HashSet::new();
      for (_, weight) in child_weights.clone() {
        unique_weights.insert(weight);
      }
      if unique_weights.len() > 1 {
        children_are_balanced = false;
      }
      println!("Current node: {}, Parent is balanced: {}, Children are balanced: {}", current_node, parent_is_balanced, children_are_balanced);
      if !parent_is_balanced && children_are_balanced {
        unbalanced_node = current_node.clone();
        unbalanced_weight = *weights.get(&current_node).unwrap() - child_weights.values().sum::<usize>();
        break;
      }
      queue.push(QueueItem {
        node: current_node.clone(),
        depth: depth,
        parent_is_balanced: parent_is_balanced,
      });
    }
    println!("Unbalanced node: {}", unbalanced_node);
    println!("Unbalanced weight: {}", unbalanced_weight);
    println!("Depth: {}", depth);
    return unbalanced_weight;
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
        let child_weight = get_weight(
            input,
            weights,
            &child,
            *new_weight,
            depth + 1,
        );
        total_weight += child_weight;
    }
    return total_weight;
}

fn main() {
    let data = read_input("input.txt");
    let root = part_one(&data.connections);
    println!("Part one: {}", root.clone());
    println!(
        "Part two: {}",
        part_two(&data.connections, &data.weights, root)
    );
}
