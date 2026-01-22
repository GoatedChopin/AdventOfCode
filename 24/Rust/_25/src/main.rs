use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Lock {
    columns: Vec<usize>,
    max_height: usize,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Key {
    columns: Vec<usize>,
}

impl Key {
    fn new(columns: Vec<usize>) -> Self {
        Self { columns }
    }

    fn unlock(self, lock: &Lock) -> bool {
        self.columns
            .iter()
            .zip(lock.columns.iter())
            .all(|(key_column, lock_column)| key_column + lock_column <= lock.max_height)
    }
}

impl Lock {
    fn new(columns: Vec<usize>, max_height: usize) -> Self {
        Self {
            columns,
            max_height,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum InputState {
    Lock,
    Key,
    Fresh,
}

fn read_input(path: &str) -> (HashSet<Lock>, HashSet<Key>) {
    let mut locks = HashSet::new();
    let mut keys = HashSet::new();

    let lines = fs::read_to_string(path).unwrap();
    let mut state = InputState::Fresh;
    let mut max_height = 0;
    let mut current_columns = Vec::new();
    for line in lines.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        if chars.is_empty() {
            match state {
                InputState::Lock => {
                    locks.insert(Lock::new(current_columns, max_height));
                    current_columns = Vec::new();
                }
                InputState::Key => {
                    keys.insert(Key::new(current_columns));
                    current_columns = Vec::new();
                }
                InputState::Fresh => {
                    current_columns = Vec::new();
                }
            }
            state = InputState::Fresh;
        }
        chars.iter().enumerate().for_each(|(i, c)| {
            if i == 0 && state == InputState::Fresh {
                state = if *c == '#' {
                    InputState::Lock
                } else {
                    InputState::Key
                };
            }
            if current_columns.len() <= i {
                current_columns.push(0);
            }
            if *c == '#' {
                current_columns[i] += 1;
            }
        });
        if locks.len() == 0 && keys.len() == 0 {
            max_height += 1;
        }
    }

    match state {
        InputState::Lock => {
            locks.insert(Lock::new(current_columns, max_height));
        }
        InputState::Key => {
            keys.insert(Key::new(current_columns));
        }
        _ => {}
    }

    (locks, keys)
}

fn part_one(locks: &HashSet<Lock>, keys: &HashSet<Key>) -> usize {
    let mut compatible = HashSet::new();
    for lock in locks.iter() {
        for key in keys.iter() {
            if key.clone().unlock(lock) {
                compatible.insert((key.clone(), lock));
            }
        }
    }
    compatible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (locks, keys) = read_input("test.txt");
        println!("{:?}", locks);
        println!("{:?}", keys);
        assert_eq!(part_one(&locks, &keys), 3);
    }
}

fn main() {
    let (locks, keys) = read_input("input.txt");
    println!("Part one: {}", part_one(&locks, &keys));
}
