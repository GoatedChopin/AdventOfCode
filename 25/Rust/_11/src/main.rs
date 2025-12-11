use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn read_input(file_path: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .for_each(|line| {
            let (name, children) = line.split_once(": ").unwrap();
            let children = children
                .split(" ")
                .map(|child| child.trim())
                .collect::<Vec<&str>>();
            graph.insert(
                name.to_string(),
                children
                    .iter()
                    .map(|child| child.to_string())
                    .collect::<Vec<String>>(),
            );
        });
    graph
}

fn traverse(
    graph: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    memo: &mut HashMap<(String, String), usize>,
) -> usize {
    let mut paths = 0;
    if start == end {
        paths += 1;
        return paths;
    }
    if !graph.contains_key(start) {
        return paths;
    }
    for child in &graph[start] {
        if child == end {
            paths += 1;
            continue;
        }
        let child_to_end = memo.get(&(child.to_string(), end.to_string()));
        if let Some(child_to_end) = child_to_end {
            paths += child_to_end;
        } else {
            let result = traverse(graph, child, end, memo);
            memo.insert((child.to_string(), end.to_string()), result);
            paths += result;
        }
    }
    paths
}

fn part_one(graph: &HashMap<String, Vec<String>>) -> usize {
    traverse(graph, "you", "out", &mut HashMap::new())
}

struct TraverseCondensedState {
  current: String,
  visited: HashSet<String>,
  num_paths: usize,
}

fn traverse_condensed(graph: &HashMap<(String, String), usize>, start: &str, end: &str, required: &HashSet<String>) -> usize {
  let mut valid_paths = 0;
  let mut queue = VecDeque::new();
  queue.push_back(TraverseCondensedState {
    current: start.to_string(),
    visited: HashSet::from_iter([start.to_string()]),
    num_paths: 1,
  });

  while let Some(state) = queue.pop_front() {
    if state.current == end && required.is_subset(&state.visited) {
      valid_paths += state.num_paths;
      continue;
    }
    for ((from, to), num_paths) in graph.iter() {
      if state.visited.contains(to) {
        continue;
      }
      if from != &state.current {
        continue;
      }
      let mut new_visited = state.visited.clone();
      new_visited.insert(to.clone());
      queue.push_back(TraverseCondensedState {
        current: to.clone(),
        visited: new_visited,
        num_paths: state.num_paths * num_paths,
      });
    }
  }
  valid_paths
}

fn part_two(graph: &HashMap<String, Vec<String>>, required: &HashSet<String>) -> usize {
    let start = "svr";
    let end = "out";
    let mut num_paths_between = HashMap::new();
    for r_a in required.iter() {
      let start_to_r_a = traverse(graph, start, r_a, &mut HashMap::new());
      num_paths_between.insert((start.to_string(), r_a.clone()), start_to_r_a);
      let r_a_to_end = traverse(graph, r_a, end, &mut HashMap::new());
      num_paths_between.insert((r_a.clone(), end.to_string()), r_a_to_end);
      for r_b in required.iter() {
        if r_a == r_b {
          continue;
        }
        let num_paths = traverse(graph, r_a, r_b, &mut HashMap::new());
        num_paths_between.insert((r_a.clone(), r_b.clone()), num_paths);
      }
    }
    // Now that we know exactly how many paths there are between each pair of required nodes, we can calculate the total number of paths from the start to the end that include all required nodes more efficiently.
    let mut total_paths = 0;
    for r_a in required.iter() {
      for r_b in required.iter() {
        if r_a == r_b {
          continue;
        }
        let start_to_r_a = num_paths_between.get(&(start.to_string(), r_a.clone())).unwrap();
        if *start_to_r_a == 0 {
          // r_a is not reachable from start so should not be included in the total paths as a first node
          continue;
        }
        total_paths += start_to_r_a * traverse_condensed(&num_paths_between, r_a, end, required);
      }
    }
    total_paths
}

fn test() {
    let input = read_input("test_one.txt");
    assert_eq!(part_one(&input), 5);
    let input = read_input("test_two.txt");
    assert_eq!(
        part_two(
            &input,
            &HashSet::from_iter(["dac", "fft"].iter().map(|s| s.to_string()))
        ),
        2
    );
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!(
        "Part two: {}",
        part_two(
            &input,
            &HashSet::from_iter(["dac", "fft"].iter().map(|s| s.to_string()))
        )
    );
}
