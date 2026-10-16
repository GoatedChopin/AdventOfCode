use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Graph {
    edges: HashMap<(String, String), usize>,
    vertices: HashSet<String>,
}

fn read_input(input: &str) -> Graph {
    let bytes = fs::read_to_string(input).expect("Input file must exist");
    let mut edges = HashMap::new();
    let mut vertices = HashSet::new();
    for line in bytes.lines().into_iter() {
        let (locations, distance_str) = line.split_once(" = ").expect("Missing ' = '");
        let (origin, destination) = locations.split_once(" to ").expect("Missing ' to '");
        let distance: usize = distance_str.parse().expect("Bad distance value");
        edges.insert((origin.to_string(), destination.to_string()), distance);
        edges.insert((destination.to_string(), origin.to_string()), distance);
        vertices.insert(origin.to_string());
        vertices.insert(destination.to_string());
    }
    Graph { edges, vertices }
}

fn greedy_dfs(
    start: &String,
    total: Option<usize>,
    hops: Option<usize>,
    max_path_length: Option<usize>,
    graph: &Graph,
    visited: &mut Vec<String>,
) -> Option<usize> {
    if total.unwrap_or(0) >= max_path_length.unwrap_or(usize::MAX) {
        return None;
    }
    if hops.unwrap_or(0) + 1 == graph.vertices.len() {
        return total;
    }
    visited.push(start.clone());
    let mut shortest_path = None;
    if visited.len() < graph.vertices.len() {
        // Keep going
        for ((origin, destination), distance) in graph.edges.iter() {
            if *origin != *start {
                continue;
            }
            if visited.contains(destination) {
                continue;
            }
            let new_total = greedy_dfs(
                destination,
                Some(total.unwrap_or(0) + *distance),
                Some(hops.unwrap_or(0) + 1),
                max_path_length,
                graph,
                visited,
            );
            if shortest_path.is_none()
                || new_total.is_some_and(|t| t < shortest_path.unwrap_or(usize::MAX))
            {
                shortest_path = new_total;
            }
        }
    }
    visited.pop();
    shortest_path
}

fn part_one(graph: &Graph) -> usize {
    let mut shortest_path = usize::MAX;
    for vertex in graph.vertices.iter() {
        let mut visited = Vec::new();
        let path = greedy_dfs(vertex, None, None, Some(shortest_path), graph, &mut visited);
        shortest_path = shortest_path.min(path.unwrap_or(usize::MAX));
    }
    return shortest_path;
}

fn ungreedy_dfs(
    start: &String,
    total: Option<usize>,
    hops: Option<usize>,
    min_path_length: Option<usize>,
    graph: &Graph,
    visited: &mut Vec<String>,
) -> Option<usize> {
    if hops.unwrap_or(0) + 1 == graph.vertices.len()
        && total.unwrap_or(0) >= min_path_length.unwrap_or(0)
    {
        return total;
    }
    visited.push(start.clone());
    let mut longest_path = None;
    if visited.len() < graph.vertices.len() {
        // Keep going
        for ((origin, destination), distance) in graph.edges.iter() {
            if *origin != *start {
                continue;
            }
            if visited.contains(destination) {
                continue;
            }
            let new_total = ungreedy_dfs(
                destination,
                Some(total.unwrap_or(0) + *distance),
                Some(hops.unwrap_or(0) + 1),
                min_path_length,
                graph,
                visited,
            );
            if new_total.is_some_and(|t| t > longest_path.unwrap_or(0)) {
                longest_path = new_total;
            }
        }
    }
    // println!("Found path {:?} from vertices {:?}", shortest_path, visited);
    visited.pop();
    longest_path
}

fn part_two(graph: &Graph) -> usize {
    let mut longest_path = 0;
    for vertex in graph.vertices.iter() {
        let mut visited = Vec::new();
        let path = ungreedy_dfs(vertex, None, None, Some(longest_path), graph, &mut visited);
        longest_path = longest_path.max(path.unwrap_or(0));
    }
    return longest_path;
}

fn main() {
    let graph = read_input("input.txt");
    println!("Part one: {}", part_one(&graph));
    println!("Part two: {}", part_two(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let graph = read_input("test.txt");
        let p1 = part_one(&graph);
        assert_eq!(p1, 605);
    }

    #[test]
    fn test_part_two() {
        let graph = read_input("test.txt");
        let p2 = part_two(&graph);
        assert_eq!(p2, 982);
    }
}
