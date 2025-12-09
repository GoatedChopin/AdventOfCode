use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    a_index: usize,
    b_index: usize,
    distance: usize,
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
    let x_distance = absolute_compare(a.x, b.x);
    let y_distance = absolute_compare(a.y, b.y);
    let z_distance = absolute_compare(a.z, b.z);
    (x_distance * x_distance) + (y_distance * y_distance) + (z_distance * z_distance)
}

fn part_one(input: &Vec<Coordinate>, connections: usize) -> usize {
    let coordinates = input.clone();
    let mut graph = vec![vec![]; input.len()];

    // Compute the shortest connection that doesn't already exist N times.
    let mut visited = HashSet::new();
    for _ in 0..connections {
        let mut min_distance_indexes = (0, 0);
        let mut min_distance = usize::MAX;
        for c_index in 0..input.len() {
            let coord = coordinates[c_index];
            let distances: Vec<usize> = coordinates.iter().map(|c| distance(&c, &coord)).collect();
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
                    min_distance_indexes = (c_index, d_index);
                }
            }
        }
        // println!(
        //     "Connecting coord {:?} to {:?}",
        //     coordinates[min_distance_indexes.0], coordinates[min_distance_indexes.1]
        // );
        visited.insert(min_distance_indexes);
        graph[min_distance_indexes.0].push(min_distance_indexes.1);
        graph[min_distance_indexes.1].push(min_distance_indexes.0);
    }

    let mut circuits: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    for i in 0..coordinates.len() {
        let mut num_nodes = 0;
        if visited.contains(&i) {
            continue;
        }
        let mut queue = graph[i].clone();
        while queue.len() > 0 {
            let current_index = queue.pop().unwrap();
            if visited.contains(&current_index) {
                continue;
            }
            // print!("-> {:?} ", coordinates[current_index]);
            num_nodes += 1;
            visited.insert(current_index);
            let children = &graph[current_index];
            for c in 0..children.len() {
                let child = children[c];
                if visited.contains(&child) {
                    continue;
                }
                queue.push(child);
            }
        }
        circuits.insert(i, num_nodes);
        // println!("");
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

    // println!("sorted_circuits: {:?}", sorted_circuits);
    sorted_circuits[0] * sorted_circuits[1] * sorted_circuits[2]
}

fn part_two(input: &Vec<Coordinate>) -> usize {
  // Build a vec of hashsets where each set contains the index of the coordinate and represents a subgraph
  // When two subgraphs connect, merge the two sets and delete the smaller set
  let mut subgraphs = Vec::new();
  let mut node_lookup = HashMap::new();
  for i in 0..input.len() {
    subgraphs.push(HashSet::new());
    subgraphs[i].insert(i);
    node_lookup.insert(i, i);
  }
  // Precompute the distances between all coordinates as an Edge struct
  let mut edges = vec![Edge { a_index: 0, b_index: 0, distance: 0 }; (input.len() * input.len()) - input.len()]; // allocate the space up front to avoid reallocations
  let mut edge_index = 0;
  for i in 0..input.len() {
    for j in i+1..input.len() {
      if i == j {
        continue;
      }
      edges[edge_index] = Edge { a_index: i, b_index: j, distance: distance(&input[i], &input[j]) };
      edge_index += 1;
    }
  }
  edges.sort_by(|a, b| b.distance.cmp(&a.distance));
  // Iterate through the edges and merge the subgraphs if the two coordinates are in different subgraphs

  let mut subgraph_count = subgraphs.len();
  loop {
    let edge = edges.pop().unwrap();
    // print!("edge: {:?} -> ", edge);
    let a_subgraph = node_lookup.get(&edge.a_index).unwrap().clone();
    let b_subgraph = node_lookup.get(&edge.b_index).unwrap().clone();
    if a_subgraph == b_subgraph {
      continue;
    }
    let b_subgraph_members = subgraphs[b_subgraph].clone();
    subgraphs[a_subgraph].extend(&b_subgraph_members);
    for member in b_subgraph_members.iter() {
      node_lookup.insert(*member, a_subgraph);
    }
    subgraphs[b_subgraph] = HashSet::new();
    subgraph_count -= 1;
    // println!("subgraph_count: {} connecting {:?} and {:?}", subgraph_count, input[latest_edge.a_index], input[latest_edge.b_index]);
    if subgraph_count == 1 {
      let a = input[edge.a_index];
      let b = input[edge.b_index];
      return a.x * b.x;
    }
  }
}

fn test() {
    let input = read_input("test.txt");
    assert_eq!(part_one(&input, 10), 40);
    assert_eq!(part_two(&input), 25272);
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("part one: {}", part_one(&input, 1000));
    println!("part two: {}", part_two(&input));
}
