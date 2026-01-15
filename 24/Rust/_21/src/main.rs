use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Orientation {
    Tenkey,
    Arrowkey,
}

struct Pinpad {
    map: HashMap<Point, char>,
    orientation: Orientation,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_delta(drow: &isize, dcol: &isize) -> Self {
        if *drow == 0 && *dcol == -1 {
            Self::Left
        } else if *drow == 0 && *dcol == 1 {
            Self::Right
        } else if *drow == -1 && *dcol == 0 {
            Self::Up
        } else if *drow == 1 && *dcol == 0 {
            Self::Down
        } else {
            panic!("Invalid delta: ({}, {})", *drow, *dcol);
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

struct Path {
    path: String,
    turns: i32,
    steps: i32,
    current_point: Point,
    start_char: char,
    direction: Option<Direction>,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparisons: fewer turns and shorter paths get higher priority in max-heap
        let turns_cmp = other.turns.cmp(&self.turns);
        if turns_cmp != Ordering::Equal {
            return turns_cmp;
        }
        other.path.len().cmp(&self.path.len())
    }
}

impl Pinpad {
    fn new_tenkey() -> Self {
        let mut map = HashMap::new();
        map.insert(Point { row: 0, col: 0 }, '1');
        map.insert(Point { row: 0, col: 1 }, '2');
        map.insert(Point { row: 0, col: 2 }, '3');
        map.insert(Point { row: 1, col: 0 }, '4');
        map.insert(Point { row: 1, col: 1 }, '5');
        map.insert(Point { row: 1, col: 2 }, '6');
        map.insert(Point { row: 2, col: 0 }, '7');
        map.insert(Point { row: 2, col: 1 }, '8');
        map.insert(Point { row: 2, col: 2 }, '9');
        map.insert(Point { row: 3, col: 1 }, '0');
        map.insert(Point { row: 3, col: 2 }, 'A');

        Self {
            map,
            orientation: Orientation::Tenkey,
        }
    }

    fn new_arrowkey() -> Self {
        let mut map = HashMap::new();
        map.insert(Point { row: 0, col: 1 }, '^');
        map.insert(Point { row: 0, col: 2 }, 'A');
        map.insert(Point { row: 1, col: 0 }, '<');
        map.insert(Point { row: 1, col: 1 }, 'v');
        map.insert(Point { row: 1, col: 2 }, '>');

        Self {
            map,
            orientation: Orientation::Arrowkey,
        }
    }

    fn generate_min_paths(&self) -> HashMap<(char, char), Vec<String>> {
        let mut min_paths: HashMap<(char, char), Vec<String>> = HashMap::new();
        let deltas = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (point, start_char) in self.map.iter() {
            let mut queue = BinaryHeap::new();
            // Track best cost (turns, steps) to reach each point
            let mut best_cost: HashMap<Point, (i32, i32)> = HashMap::new();
            queue.push(Path {
                path: String::new(),
                turns: 0,
                steps: 0,
                start_char: *start_char,
                current_point: *point,
                direction: None,
            });
            while let Some(Path {
                path,
                turns,
                steps,
                current_point,
                start_char,
                direction,
            }) = queue.pop()
            {
                let key = (start_char, *self.map.get(&current_point).unwrap());
                
                // Check if we've seen this point with a better cost
                if let Some(&(best_turns, best_steps)) = best_cost.get(&current_point) {
                    if turns > best_turns || (turns == best_turns && steps > best_steps) {
                        continue; // Skip worse paths
                    }
                }
                
                // Record or update best cost for this point
                best_cost.insert(current_point, (turns, steps));
                
                // Add this path to the collection
                min_paths.entry(key).or_insert_with(Vec::new).push(path.clone());
                
                for (drow, dcol) in deltas.iter() {
                    let new_point = Point {
                        row: current_point.row + *drow,
                        col: current_point.col + *dcol,
                    };
                    if !self.map.contains_key(&new_point) {
                        continue;
                    }
                    let new_direction = Direction::from_delta(drow, dcol);
                    let new_turns = turns
                        + if direction.is_some() && direction.unwrap() != new_direction {
                            1
                        } else {
                            0
                        };
                    let new_steps = steps + 1;
                    
                    // Only explore if this could be an optimal or equivalent path
                    if let Some(&(best_turns, best_steps)) = best_cost.get(&new_point) {
                        if new_turns > best_turns || (new_turns == best_turns && new_steps > best_steps) {
                            continue;
                        }
                    }
                    
                    let mut new_string = path.clone();
                    new_string.push(new_direction.to_char());
                    let new_path = Path {
                        path: new_string,
                        turns: new_turns,
                        steps: new_steps,
                        start_char,
                        current_point: new_point,
                        direction: Some(new_direction),
                    };
                    queue.push(new_path);
                }
            }
        }
        min_paths
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let input = fs::read_to_string(filename).expect(&format!("Failed to read {}", filename));
    input.lines().map(|line| line.to_string()).collect()
}

fn minimum_instructions(passcode: &str, chain_of_custodcol: Vec<Pinpad>) -> String {
    let mut min_paths = HashMap::new();
    for pinpad in chain_of_custodcol.iter() {
      if min_paths.contains_key(&pinpad.orientation) {
        continue;
      }
      min_paths.insert(pinpad.orientation, pinpad.generate_min_paths());
    }
    let mut layers = Vec::new();
    layers.push(passcode.chars().into_iter().collect::<Vec<_>>());
    for pinpad in chain_of_custodcol.iter() {
      let current_layer = layers[layers.len() - 1].clone();
      // Take each pair of chars in current_layer like current_layer[i] <-> current_layer[i + 1]
      // For now, take the first equivalent path from each set
      let new_layer = current_layer.windows(2).map(|pair| {
          let paths = min_paths.get(&pinpad.orientation).unwrap().get(&(pair[0], pair[1])).unwrap();
          paths[0].clone() + "A"
      }).collect::<Vec<_>>();
      layers.push(new_layer.iter().fold(Vec::new(), |mut v, x| {v.extend(x.chars()); v}));
    }
    layers[layers.len() - 1].iter().collect::<String>()
}

fn part_one(passcodes: Vec<String>) -> String {
  passcodes.iter().map(|passcode| minimum_instructions(passcode, vec![Pinpad::new_tenkey(), Pinpad::new_arrowkey(), Pinpad::new_tenkey()])).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows() {
      let i = [1, 2, 3, 4]
        .windows(2)
        .map(|pair| (pair[0], pair[1]));
      println!("{:?}", i.collect::<Vec<_>>());
    }

    #[test]
    fn test_generate_min_paths() {
      let pinpad = Pinpad::new_tenkey();
      let min_paths = pinpad.generate_min_paths();
      println!("{:?}", min_paths);
    }

    #[test]
    fn test_generate_min_paths_arrowkey() {
      let pinpad = Pinpad::new_arrowkey();
      let min_paths = pinpad.generate_min_paths();
      println!("{:?}", min_paths);
    }

    #[test]
    fn test_part_one() {
        /*
          029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
          980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
          179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
          456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
          379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        */
        assert_eq!(
            minimum_instructions("029A", vec![Pinpad::new_tenkey(), Pinpad::new_arrowkey(), Pinpad::new_arrowkey(), Pinpad::new_arrowkey()]),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        );
    }
}

fn main() {
    let passcodes = read_input("input.txt");
    println!("{}", part_one(passcodes));
}
