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
    steps: i32,
    current_point: Point,
    start_char: char,
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
        // Reverse comparison: shorter paths get higher priority in max-heap
        other.steps.cmp(&self.steps)
    }
}

impl Pinpad {
    fn new_tenkey() -> Self {
        let mut map = HashMap::new();
        // AoC numpad layout:
        // 7 8 9
        // 4 5 6
        // 1 2 3
        // X 0 A  (X is empty/invalid at row 3, col 0)
        map.insert(Point { row: 0, col: 0 }, '7');
        map.insert(Point { row: 0, col: 1 }, '8');
        map.insert(Point { row: 0, col: 2 }, '9');
        map.insert(Point { row: 1, col: 0 }, '4');
        map.insert(Point { row: 1, col: 1 }, '5');
        map.insert(Point { row: 1, col: 2 }, '6');
        map.insert(Point { row: 2, col: 0 }, '1');
        map.insert(Point { row: 2, col: 1 }, '2');
        map.insert(Point { row: 2, col: 2 }, '3');
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
            // Track minimum steps to reach each point (we want ALL shortest paths)
            let mut min_steps: HashMap<Point, i32> = HashMap::new();
            queue.push(Path {
                path: String::new(),
                steps: 0,
                start_char: *start_char,
                current_point: *point,
            });
            while let Some(Path {
                path,
                steps,
                current_point,
                start_char,
            }) = queue.pop()
            {
                let key = (start_char, *self.map.get(&current_point).unwrap());
                
                // Check if we've seen this point with fewer steps
                if let Some(&best_steps) = min_steps.get(&current_point) {
                    if steps > best_steps {
                        continue; // Skip longer paths
                    }
                }
                
                // Record minimum steps for this point
                min_steps.insert(current_point, steps);
                
                // Add this path to the collection (avoiding duplicates)
                let paths = min_paths.entry(key).or_insert_with(Vec::new);
                if !paths.contains(&path) {
                    paths.push(path.clone());
                }
                
                for (drow, dcol) in deltas.iter() {
                    let new_point = Point {
                        row: current_point.row + *drow,
                        col: current_point.col + *dcol,
                    };
                    if !self.map.contains_key(&new_point) {
                        continue;
                    }
                    let new_direction = Direction::from_delta(drow, dcol);
                    let new_steps = steps + 1;
                    
                    // Only explore if this could be a shortest path
                    if let Some(&best_steps) = min_steps.get(&new_point) {
                        if new_steps > best_steps {
                            continue;
                        }
                    }
                    
                    let mut new_string = path.clone();
                    new_string.push(new_direction.to_char());
                    let new_path = Path {
                        path: new_string,
                        steps: new_steps,
                        start_char,
                        current_point: new_point,
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
    // Pre-compute all min paths for each pinpad orientation
    let mut min_paths_cache: HashMap<Orientation, HashMap<(char, char), Vec<String>>> = HashMap::new();
    for pinpad in chain_of_custodcol.iter() {
        if !min_paths_cache.contains_key(&pinpad.orientation) {
            min_paths_cache.insert(pinpad.orientation, pinpad.generate_min_paths());
        }
    }
    
    // Collect orientations in order
    let orientations: Vec<Orientation> = chain_of_custodcol.iter().map(|p| p.orientation).collect();
    
    // Memoization for best path choice: (from_char, to_char, layer_idx) -> best path string
    let mut len_memo: HashMap<(char, char, usize), usize> = HashMap::new();
    let mut path_memo: HashMap<(char, char, usize), String> = HashMap::new();
    
    // Compute the minimum final length for a single transition (and cache best path)
    fn compute_best(
        from: char,
        to: char,
        layer_idx: usize,
        orientations: &[Orientation],
        min_paths_cache: &HashMap<Orientation, HashMap<(char, char), Vec<String>>>,
        len_memo: &mut HashMap<(char, char, usize), usize>,
        path_memo: &mut HashMap<(char, char, usize), String>,
    ) -> (usize, String) {
        // Base case: past all layers
        if layer_idx >= orientations.len() {
            return (0, String::new());
        }
        
        // Check memo
        if let Some(&cached_len) = len_memo.get(&(from, to, layer_idx)) {
            let cached_path = path_memo.get(&(from, to, layer_idx)).unwrap().clone();
            return (cached_len, cached_path);
        }
        
        let orientation = orientations[layer_idx];
        let paths_map = min_paths_cache.get(&orientation).unwrap();
        let paths = paths_map.get(&(from, to)).unwrap();
        
        // Try each path option and find the one with minimum expansion
        let mut best_len = usize::MAX;
        let mut best_path = String::new();
        
        for path in paths {
            let full_path = path.clone() + "A";
            let chars: Vec<char> = std::iter::once('A').chain(full_path.chars()).collect();
            
            let mut total_len = 0;
            let mut expanded = String::new();
            
            for pair in chars.windows(2) {
                let (sub_len, sub_path) = compute_best(
                    pair[0],
                    pair[1],
                    layer_idx + 1,
                    orientations,
                    min_paths_cache,
                    len_memo,
                    path_memo,
                );
                total_len += sub_len;
                expanded.push_str(&sub_path);
            }
            
            // At the final layer, the length is the actual characters
            if layer_idx == orientations.len() - 1 {
                total_len = full_path.len();
                expanded = full_path;
            }
            
            if total_len < best_len {
                best_len = total_len;
                best_path = expanded;
            }
        }
        
        len_memo.insert((from, to, layer_idx), best_len);
        path_memo.insert((from, to, layer_idx), best_path.clone());
        (best_len, best_path)
    }
    
    // For the passcode, build the final result
    let chars: Vec<char> = std::iter::once('A').chain(passcode.chars()).collect();
    let mut result = String::new();
    for pair in chars.windows(2) {
        let (_, sub_path) = compute_best(
            pair[0],
            pair[1],
            0,
            &orientations,
            &min_paths_cache,
            &mut len_memo,
            &mut path_memo,
        );
        result.push_str(&sub_path);
    }
    result
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
        // Note: The expected outputs above are for numpad + 2 arrowkeys (3 keypads total)
        let result = minimum_instructions("029A", vec![Pinpad::new_tenkey(), Pinpad::new_arrowkey(), Pinpad::new_arrowkey()]);
        println!("Result length: {}, expected: 68", result.len());
        println!("Result: {}", result);
        assert_eq!(result.len(), 68);
    }
}

fn main() {
    let passcodes = read_input("input.txt");
    println!("{}", part_one(passcodes));
}
