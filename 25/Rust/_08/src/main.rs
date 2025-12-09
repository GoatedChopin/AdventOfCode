use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

fn read_input(path: &str) -> Vec<Coordinate> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();
            Coordinate {
                x: parts[0].parse::<usize>().unwrap(),
                y: parts[1].parse::<usize>().unwrap(),
                z: parts[2].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Coordinate>>()
}

fn absolute_compare(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

fn distance(a: &Coordinate, b: &Coordinate) -> usize {
    absolute_compare(a.x, b.x) + absolute_compare(a.y, b.y) + absolute_compare(a.z, b.z)
}

fn part_one(input: &Vec<Coordinate>, connections: usize) -> usize {
    let mut graph = HashMap::new();
    let coordinates = input.clone();

    // Compute the shortest connection that doesn't already exist N times.
    let mut visited = HashSet::new();
    for _ in 0..connections {
        for c_index in 0..input.len() {
            let coord = coordinates[c_index];
            // default value for graph
            if !graph.contains_key(&c_index) {
                graph.insert(c_index.clone(), Vec::new());
            }
            let distances: Vec<usize> = coordinates.iter().map(|c| distance(&c, &coord)).collect();
            let mut min_distance_index = 0;
            let mut min_distance = usize::MAX;
            for d_index in 0..distances.len() {
                let distance = distances[d_index];
                if distance == 0 {
                    // Can't connect a generator to itself
                    continue;
                }
                if visited.contains(&(c_index, d_index)) || visited.contains(&(d_index, c_index)) {
                    // Can't create duplicate connections
                    continue;
                }
                if distance < min_distance {
                    min_distance = distance;
                    min_distance_index = d_index;
                }
            }

            visited.insert((c_index, min_distance_index));
            if !graph.contains_key(&c_index) {
                graph.insert(c_index.clone(), vec![]);
            }
            let current_connections = graph.get_mut(&c_index).unwrap();
            current_connections.push(min_distance_index);

            if !graph.contains_key(&min_distance_index) {
                graph.insert(min_distance_index.clone(), vec![]);
            }
            let current_connections = graph.get_mut(&min_distance_index).unwrap();
            current_connections.push(c_index);
        }
    }

    // Traverse the graph starting from each coordinate.
    // - If we reach a position we've already visited on the first node, skip the current node
    // - If we reach a position we've already visited on the second..Nth node, assign the size of the graph
    let mut circuits: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    for i in 0..coordinates.len() {
        let mut num_nodes = 1;
        if visited.contains(&i) {
            continue;
        }
        let mut queue = Vec::new();
        let starting_items = graph.get(&i).unwrap();
        for s_index in 0..starting_items.len() {
            queue.push(starting_items[s_index])
        }
        while queue.len() > 0 {
            let current_index = queue.pop().unwrap();
            num_nodes += 1;
            visited.insert(current_index);
            let children = graph.get(&current_index).unwrap();
            for c in 0..children.len() {
                if visited.contains(&c) {
                    continue;
                }
                queue.push(children[c]);
            }
        }
        circuits.insert(i, num_nodes);
    }

    let mut sorted_circuits = coordinates
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let num_nodes = circuits.get(&i);
            match num_nodes {
                Some(num_nodes) => {
                    return num_nodes.clone();
                }
                None => {
                    return 1;
                }
            }
        })
        .collect::<Vec<usize>>();
    sorted_circuits.sort_by(|a, b| b.cmp(a));

    sorted_circuits[0] * sorted_circuits[1] * sorted_circuits[2]
}

fn test() {
    let input = read_input("test.txt");
    assert_eq!(part_one(&input, 10), 40);
}

fn main() {
    test();
}
