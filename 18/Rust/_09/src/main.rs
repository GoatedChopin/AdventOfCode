use std::error::Error;

#[derive(Clone, Copy)]
struct Node {
    value: usize,
    prev: usize,
    next: usize,
}

impl Node {
    fn new(value: usize, prev: usize, next: usize) -> Self {
        Self {
            value,
            prev,
            next,
        }
    }
}

struct List {
    nodes: Vec<Node>,
    current: usize,
}

enum InsertError {
    OutOfBoundsError
}

impl List {
    fn new() -> Self {
        Self {
            nodes: vec![],
            current: 0,
        }
    }

    fn get(&self, index: usize) -> Node {
        return self.nodes[index % self.nodes.len()];
    }

    fn append(&mut self, value: usize) -> Result<(), InsertError> {
        self.nodes.push(Node::new(value, self.nodes.len() - 1, self.nodes.len() + 1));
        Ok(())
    }

    fn insert(&mut self, value: usize, position: usize) -> Result<(), InsertError> {
        if position > self.nodes.len() {
            return Err(InsertError::OutOfBoundsError)
        }

        if position == self.nodes.len() {
            self.append(value);
            return Ok(());
        }

        let mut node = Node::new(value, 0, 0);

        
        Ok(())
    }
}

fn part_one(players: usize, last_marble: usize) -> usize {
    0
}

fn main() {
    let players = 419;
    let last_marble = 71052;
    println!("Part one: {}", part_one(players, last_marble));
}
