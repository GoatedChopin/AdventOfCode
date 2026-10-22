use std::collections::HashMap;
use std::fs;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Person(String);

#[derive(Debug, Clone)]
struct Rule {
    a: Person,
    b: Person,
    change: isize,
}

#[derive(Debug, Clone)]
struct Table {
    position_vec: Vec<Person>,
    position_map: HashMap<Person, usize>,
    rules: Vec<Rule>,
    rng: ThreadRng,
}

impl Table {
    fn eval(&self) -> isize {
        let mut happiness = 0;
        for (person, position) in self.position_map.iter() {
            self.rules
                .iter()
                .filter(|rule| *person == rule.a)
                .for_each(|rule| {
                    let left = if *position > 0 {
                        position - 1
                    } else {
                        self.position_map.len() - 1
                    };
                    let right = if *position < self.position_map.len() - 1 {
                        position + 1
                    } else {
                        0
                    };
                    if self.position_vec[left] == rule.b || self.position_vec[right] == rule.b {
                        // println!("{:?} <-> {:?}: {}", rule.a, rule.b, rule.change);
                        happiness += rule.change;
                    }
                })
        }
        happiness
    }

    fn swap(&mut self, i: usize, j: usize) {
        let i_person = self.position_vec[i].clone();
        let j_person = self.position_vec[j].clone();
        self.position_vec.swap(i, j);
        self.position_map.insert(i_person, j);
        self.position_map.insert(j_person, i);
    }

    fn optimize(&mut self) -> isize {
        let mut can_optimize = true;
        let mut best_happiness = self.eval();
        while can_optimize {
            can_optimize = false;
            let mut current_happiness = best_happiness.max(self.eval());
            for current_position in 0..self.position_vec.len() {
                let person = self.position_vec[current_position].clone();
                for position in 0..self.position_vec.len() {
                    if self.position_vec[position] != person {
                        self.swap(current_position, position);
                        let new_happiness = self.eval();
                        best_happiness = best_happiness.max(new_happiness);
                        if new_happiness > current_happiness {
                            can_optimize = true;
                            current_happiness = new_happiness;
                            break;
                        } else {
                            self.swap(current_position, position);
                        }
                    }
                }
            }
        }
        best_happiness
    }

    fn randomize(&mut self) {
        self.position_vec.shuffle(&mut self.rng);
        for (i, person) in self.position_vec.iter().enumerate() {
            self.position_map.insert(person.clone(), i);
        }
    }
}

fn read_input(path: &str) -> Table {
    let input = fs::read_to_string(path).expect("Input file must exist");
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let a = Person(words[0].to_string());
            let b = Person(words[10].to_string());
            let mut change = words[3].parse().expect("Bad happiness value");
            let negative = words[2] == "lose";
            if negative {
                change *= -1;
            }
            Rule { a, b, change }
        })
        .collect();
    let position_vec = rules.iter().fold(Vec::new(), |mut acc, rule| {
        if !acc.contains(&rule.a) {
            acc.push(rule.a.clone());
        }
        if !acc.contains(&rule.b) {
            acc.push(rule.b.clone());
        }
        acc
    });
    let position_map =
        position_vec
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, person)| {
                acc.insert(person.clone(), i);
                acc
            });

    Table {
        position_map,
        position_vec,
        rules,
        rng: rand::rng(),
    }
}

fn part_one(table: Table) -> isize {
    let mut table = table.clone();
    let mut best_happiness = table.eval();
    for i in 0..10000 {
        table.optimize();
        best_happiness = best_happiness.max(table.eval());
        table.randomize();
    }
    best_happiness
}

fn main() {
    // The . periods at the end of each line were throwing the solution off.
    // Randomly shuffling the table a bunch of time is pretty grug-brained but it worked for me.
    let mut table = read_input("input.txt");
    println!("Part one: {}", part_one(table.clone()));
    table
        .position_map
        .insert(Person("Colby".to_string()), table.position_vec.len());
    table.position_vec.push(Person("Colby".to_string()));
    println!("Part two: {}", part_one(table));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval() {
        let mut table = read_input("test.txt");
        table.position_vec = ["Alice", "Bob", "Carol", "David"]
            .iter()
            .map(|name| Person(name.to_string()))
            .collect();

        table.position_map.clear();
        for (i, person) in table.position_vec.iter().enumerate() {
            table.position_map.insert(person.clone(), i);
        }

        assert_eq!(table.eval(), 330);
    }

    #[test]
    fn test_part_one() {
        let table = read_input("test.txt");
        assert_eq!(part_one(table), 330);
    }
}
