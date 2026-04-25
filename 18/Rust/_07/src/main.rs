use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Clone, Debug)]
struct DAG {
    // Each key must wait for all chars val to complete for completing
    prereqs: HashMap<char, HashSet<char>>,
    has_run: HashSet<char>,
}

impl DAG {
    fn from_deps(v: Vec<(char, char)>) -> Self {
        let mut prereqs: HashMap<char, HashSet<char>> = HashMap::new();
        v.into_iter().for_each(|(parent, child)| {
            let existing = prereqs.get_mut(&child);
            match existing {
                Some(p) => {
                    p.insert(parent);
                }
                None => {
                    prereqs.insert(child, HashSet::from([parent]));
                }
            }
            if !prereqs.contains_key(&parent) {
                prereqs.insert(parent, HashSet::new());
            }
        });

        Self {
            prereqs,
            has_run: HashSet::new(),
        }
    }

    fn next_unblocked(&self) -> Option<char> {
        self.prereqs
            .iter()
            .filter(|(k, v)| v.is_empty() && !self.has_run.contains(k))
            .map(|(k, _)| *k)
            .min()
    }

    fn get_unblocked_nodes(&self) -> Vec<char> {
        let mut unblocked: Vec<char> = self
            .prereqs
            .iter()
            .filter(|(k, v)| {
                if v.is_empty() && !self.has_run.contains(k) {
                    return true;
                }
                false
            })
            .map(|(k, _)| *k)
            .collect();
        unblocked.sort();
        unblocked
    }

    fn run(&mut self, c: char) {
        self.has_run.insert(c);
        self.prereqs.iter_mut().for_each(|(_, v)| {
            if v.contains(&c) {
                v.remove(&c);
            }
        });
    }
}

fn get_char_cost(c: char) -> usize {
    (c as u8 - b'A' + 1) as usize
}

fn read_input(file_path: &str) -> DAG {
    let input = fs::read_to_string(file_path).expect("Bad input path");
    let deps: Vec<(char, char)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let parts: Vec<char> = line.chars().collect();
            (parts[5], parts[36])
        })
        .collect();

    DAG::from_deps(deps)
}

fn part_one(dag: DAG) -> String {
    let mut dag = dag.clone();
    println!("{:?}", dag);
    let mut run_order: Vec<char> = Vec::new();
    while dag.has_run.len() < dag.prereqs.len() {
        let new_char = dag.next_unblocked();
        match new_char {
            Some(c) => {
                dag.run(c);
                run_order.push(c);
            }
            None => {
                break;
            }
        }
    }
    run_order.into_iter().collect()
}

fn part_two(dag: DAG, num_workers: usize, job_min_cost: usize) -> usize {
    let mut seconds = 0;
    let mut dag = dag.clone();
    let mut run_order: Vec<char> = Vec::new();
    let mut active_workers = 0;
    let mut worker_queue: HashMap<usize, Vec<char>> = HashMap::new();
    while run_order.len() < dag.prereqs.len() || worker_queue.len() > 0 {
        seconds += 1;
        match worker_queue.get(&seconds) {
            Some(v) => {
                v.iter().for_each(|c| dag.run(*c));
                run_order.extend(v);
                active_workers -= v.len();
                worker_queue.remove(&seconds);
            }
            _ => {}
        }
        if worker_queue.len() == num_workers {
            continue;
        }

        while active_workers < num_workers {
            let new_char = dag.next_unblocked();
            match new_char {
                Some(c) => {
                    let cost = get_char_cost(c) + job_min_cost;
                    println!("Running job {} for {} seconds, done at {}", c, cost, seconds + cost);
                    match worker_queue.get_mut(&(seconds + cost)) {
                        Some(v) => {
                            v.push(c);
                        }
                        None => {
                            worker_queue.insert(seconds + cost, Vec::from([c]));
                        }
                    }
                    dag.prereqs.remove(&c);
                }
                None => {
                    println!("No new jobs available, idling workers: {}", num_workers - active_workers);
                    break;
                }
            }
            active_workers += 1;
        }
    }

    match seconds == 0 {
        true => seconds,
        false => seconds - 1,
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input, 5, 60));
}

#[cfg(test)]
mod test {
    use std::fs::read;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), "CABDFE")
    }

    #[test]
    fn test_get_char_cost() {
        assert_eq!(get_char_cost('A'), 1);
        assert_eq!(get_char_cost('Z'), 26);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("test.txt");
        assert_eq!(part_two(input, 2, 0), 15);
    }
}
