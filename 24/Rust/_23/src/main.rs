use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

struct Network {
    connections: Vec<(String, String)>,
    computers: HashMap<String, HashSet<String>>,
}

fn read_input(path: &str) -> Network {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut computers = HashMap::new();
    let mut connections = Vec::new();
    contents.lines().for_each(|line| {
        let parts = line.split("-").collect::<Vec<&str>>();
        computers.insert(parts[0].to_string(), HashSet::from([parts[0].to_string(), parts[1].to_string()]));
        computers.insert(parts[1].to_string(), HashSet::from([parts[0].to_string(), parts[1].to_string()]));
        connections.push((parts[0].to_string(), parts[1].to_string()));
    });
    for (c1, c2) in connections.iter() {
        computers.get_mut(c1).unwrap().insert(c2.clone());
        computers.get_mut(c2).unwrap().insert(c1.clone());
    }
    Network {
        connections,
        computers,
    }
}

struct SearchState {
    current_node: String,
    visited: HashSet<String>,
    first_node: String,
}

fn part_one(network: &mut Network) -> usize {
    for (c1, c2) in network.connections.iter() {
        network.computers.get_mut(c1).unwrap().insert(c2.clone());
        network.computers.get_mut(c2).unwrap().insert(c1.clone());
    }
    let potential_starts = network
        .computers
        .keys()
        .filter(|k| k.starts_with("t"))
        .collect::<Vec<&String>>();

    let mut solutions = HashSet::new();
    let mut queue = VecDeque::from(
        potential_starts
            .into_iter()
            .map(|s| SearchState {
                current_node: s.clone(),
                visited: HashSet::from([s.clone()]),
                first_node: s.clone(),
            })
            .collect::<Vec<SearchState>>(),
    );
    let mut visited = HashSet::new();
    while queue.len() > 0 {
        let state = queue.pop_front().unwrap();
        visited.insert(state.current_node.clone());
        if state.visited.len() == 3
            && network
                .computers
                .get(&state.current_node)
                .unwrap()
                .contains(&state.first_node)
        {
            let mut key = state.visited.iter().collect::<Vec<&String>>();
            key.sort();
            solutions.insert(
                key.iter()
                    .map(|s| (*s).clone())
                    .collect::<Vec<String>>()
                    .join("-"),
            );
        }
        if state.visited.len() >= 3 {
            continue;
        }
        for neighbor in network
            .computers
            .get(&state.current_node)
            .unwrap()
            .iter()
            .filter(|n| !state.visited.contains(*n))
        {
            let mut new_visited = state.visited.clone();
            new_visited.insert(neighbor.clone());
            queue.push_back(SearchState {
                current_node: neighbor.clone(),
                visited: new_visited,
                first_node: state.first_node.clone(),
            });
        }
    }
    // solutions.iter().for_each(|s| println!("{}", s));
    solutions.len()
}

fn is_densely_connected(neighbors: &HashSet<String>, network: &Network) -> bool {
    for neighbor in neighbors {
        let neighbor_neighbors = network.computers.get(neighbor).unwrap();
        if !neighbor_neighbors.is_superset(neighbors) {
            return false;
        }
    }
    true
}

fn part_two(network: &mut Network) -> String {
    for (c1, c2) in network.connections.iter() {
        network.computers.get_mut(c1).unwrap().insert(c2.clone());
        network.computers.get_mut(c2).unwrap().insert(c1.clone());
    }
    let mut graph_size = network
        .computers
        .iter()
        .max_by_key(|(_, v)| v.len())
        .unwrap()
        .1
        .len();
    let mut current_best = 0;
    let mut current_best_subset = HashSet::new();
    while graph_size > 1 {
        // Check all nodes with graph size X to see if any of them are fully connected to one another
        let nodes_with_graph_size_x = network
            .computers
            .iter()
            .filter(|(_, v)| v.len() == graph_size)
            .collect::<Vec<(&String, &HashSet<String>)>>();
        for (_, neighbors) in nodes_with_graph_size_x {
            let mut neighbors_ordered_by_density = neighbors.iter().collect::<Vec<&String>>();
            neighbors_ordered_by_density.sort_by_key(|n| {
                let neighbor_neighbors = network.computers.get(*n).unwrap();
                // We want to pop the least connected neighbors first, by the number of neighbor connections they're missing
                // We want the elements with least common connections at the end of the Vector
                - (neighbor_neighbors.intersection(neighbors).count() as i32)
            });

            let mut neighbor_subset = neighbors.clone();
            while neighbors_ordered_by_density.len() > current_best {
                if is_densely_connected(&neighbor_subset, network) {
                    current_best = neighbors_ordered_by_density.len();
                    current_best_subset = neighbor_subset.clone();
                    break;
                }
                let neighbor_to_remove = neighbors_ordered_by_density.pop().unwrap();
                neighbor_subset.remove(neighbor_to_remove);
            }
        }
        graph_size -= 1;
    }
    let mut largest_dense_graph = current_best_subset.into_iter().collect::<Vec<String>>();
    largest_dense_graph.sort();
    largest_dense_graph.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let mut input = read_input("test.txt");
        assert_eq!(part_one(&mut input), 7);
    }

    #[test]
    fn test_is_densely_connected() {
        let input = read_input("test.txt");
        let neighbors = HashSet::from(["ta".to_string(), "ka".to_string(), "co".to_string(), "de".to_string()]);
        assert!(is_densely_connected(&neighbors, &input));
    }

    #[test]
    fn test_part_two() {
        let mut input = read_input("test.txt");
        assert_eq!(part_two(&mut input), "co,de,ka,ta");
    }
}

fn main() {
    let mut input = read_input("input.txt");
    println!("{}", part_one(&mut input));
    let mut input = read_input("input.txt");
    println!("{}", part_two(&mut input));
}
