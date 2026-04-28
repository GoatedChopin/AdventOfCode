enum Rotation {
    Clockwise(usize),
    CounterClockwise(usize),
}

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
            nodes: vec![Node::new(0, 0, 0)],
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

    fn insert(&mut self, value: usize, rotation: Rotation) -> Result<(), InsertError> {
        if self.nodes.len() == 0 {
          self.nodes.push(Node::new(value, 0, 0));
          return Ok(());
        }
        let (prev, next) = match rotation {
            Rotation::Clockwise(steps) => {
                let prev = self.clockwise(steps);
                let next = self.clockwise(steps + 1);
                (prev, next)
            }
            Rotation::CounterClockwise(steps) => {
                let prev = self.counter_clockwise(steps);
                let next = self.counter_clockwise(steps + 1);
                (prev, next)
            }
        };
        let node = Node::new(value, prev, next);
        self.nodes.push(node);

        let new_idx = self.nodes.len() - 1;
        self.nodes[prev].next = new_idx;
        self.nodes[next].prev = new_idx;
        self.current = new_idx;

        Ok(())
    }

    fn take(&mut self, rotation: Rotation) -> usize {
        let node_to_take = match rotation {
          Rotation::Clockwise(steps) => {
            self.clockwise(steps)
          }
          Rotation::CounterClockwise(steps) => {
            self.counter_clockwise(steps)
          }
        };

        let prev = self.nodes[node_to_take].prev;
        let next = self.nodes[node_to_take].next;

        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
        let value = self.nodes[node_to_take].value;
        
        self.current = next;

        value
    }

    fn counter_clockwise(&self, steps: usize) -> usize {
        let mut current = self.current;
        for _ in 0..steps {
            current = self.nodes[current].prev;
        }
        current
    }

    fn clockwise(&self, steps: usize) -> usize {
        let mut current = self.current;
        for _ in 0..steps {
            current = self.nodes[current].next;
        }
        current
    }
}

fn part_one(players: usize, last_marble: usize) -> usize {
  let mut list = List::new();
  let mut scores = vec![0; players];
  for i in 1..=last_marble {
    if i % 23 == 0 {
      let score = list.take(Rotation::CounterClockwise(7));
      scores[(i - 1) % players] += i + score;
      continue;
    }
    let result = list.insert(i, Rotation::Clockwise(1));
    if result.is_err() {
      println!("Error inserting marble: {}", i);
      return 0;
    }
  }

  scores.into_iter().max().unwrap()
}

fn main() {
    let players = 419;
    let last_marble = 71052;
    println!("Part one: {}", part_one(players, last_marble));
    println!("Part two: {}", part_one(players, last_marble * 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
      /*
      10 players; last marble is worth 1618 points: high score is 8317
      13 players; last marble is worth 7999 points: high score is 146373
      17 players; last marble is worth 1104 points: high score is 2764
      21 players; last marble is worth 6111 points: high score is 54718
      30 players; last marble is worth 5807 points: high score is 37305
       */
        assert_eq!(part_one(9, 25), 32);
        assert_eq!(part_one(10, 1618), 8317);
        assert_eq!(part_one(13, 7999), 146373);
        assert_eq!(part_one(17, 1104), 2764);
        assert_eq!(part_one(21, 6111), 54718);
        assert_eq!(part_one(30, 5807), 37305);
    }
}