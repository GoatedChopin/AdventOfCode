use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn read_input(path: &str) -> HashMap<usize, HashSet<usize>> {
    let mut s = fs::read_to_string(path).expect("File must exist");
    s = s.replace(" ", "");
    let mut map = HashMap::new();
    s.trim().lines().for_each(|line| {
        let Some((key, values)) = line.trim().split_once("<->") else {
            panic!("Bad input line")
        };
        let ukey: usize = key.parse().expect("Bad key value");
        let uvalues: HashSet<usize> = values
            .split(",")
            .map(|v| v.parse().expect("Bad value"))
            .collect();
        map.insert(ukey, uvalues);
    });
    map
}

fn group(start: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut queue = Vec::from([start]);
    while !queue.is_empty() {
        let Some(current) = queue.pop() else {
            continue;
        };
        let Some(connections) = graph.get(&current) else {
            continue;
        };
        for connection in connections.iter() {
            if visited.contains(connection) {
                continue;
            }
            visited.insert(*connection);
            queue.push(*connection);
        }
    }
    visited
}

fn distinct_groups(graph: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut groups = 0;
    let mut total_visited = HashSet::new();
    for (key, _) in graph.iter() {
        let g = group(*key, graph);
        let already_visited: Vec<usize> = total_visited.intersection(&g).map(|u| *u).collect();
        if already_visited.len() == 0 {
            groups += 1;
        }
        total_visited.extend(&g);
    }
    groups
}

fn main() {
    let graph = read_input("input.txt");
    println!("Part one: {}", group(0, &graph).len());
    println!("Part two: {}", distinct_groups(&graph));
}
