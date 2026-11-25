use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    fn move_by(&self, coord: (usize, usize)) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (coord.0.saturating_sub(1), coord.1),
            Down => (coord.0 + 1, coord.1),
            Left => (coord.0, coord.1.saturating_sub(1)),
            Right => (coord.0, coord.1 + 1),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum NetworkPath {
    Vertical,
    Horizontal,
    Crossroad,
}

impl NetworkPath {
    fn matches(&self, direction: Direction) -> bool {
        use Direction::*;
        use NetworkPath::*;
        match (self, direction) {
            (Vertical | Crossroad, Up | Down) => true,
            (Horizontal | Crossroad, Left | Right) => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for NetworkPath {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            '+' => Ok(Self::Crossroad),
            _ => Err("Bad input char for NetworkPath"),
        }
    }
}

#[derive(Clone)]
struct Network {
    entry: (usize, usize),
    steps: usize,
    grid: Vec<Vec<char>>,
    paths: HashMap<(usize, usize), NetworkPath>,
    letter_path: String,
}

impl Network {
    fn neighbors(&self, coord: (usize, usize)) -> Vec<(Direction, (usize, usize))> {
        get_neighbors(coord, &self.grid)
            .into_iter()
            .filter(|(direction, neighbor)| {
                let Some(path) = self.paths.get(neighbor) else {
                    return false;
                };
                match (direction, path) {
                    (
                        Direction::Up | Direction::Down,
                        NetworkPath::Crossroad | NetworkPath::Vertical,
                    ) => true,
                    (
                        Direction::Left | Direction::Right,
                        NetworkPath::Crossroad | NetworkPath::Horizontal,
                    ) => true,
                    _ => false,
                }
            })
            .collect()
    }

    fn next_direction(
        &self,
        coord: (usize, usize),
        current_direction: Direction,
    ) -> Option<Direction> {
        let mut dead_end = false;
        let mut current = coord;
        let mut direction = None;
        let mut needs_confirmation = false;
        while !dead_end {
            current = current_direction.move_by(current);
            let straight_ahead_path = self.paths.get(&current);
            if let Some(p) = straight_ahead_path {
                let is_match = p.matches(current_direction);
                if is_match && needs_confirmation {
                    break;
                } else {
                    direction = Some(current_direction);
                    needs_confirmation = true;
                }
            } else {
                dead_end = true;
            }
        }
        // If we didn't confirm the current direction, we're probably sitting on a turn
        if direction.is_none() || (direction.is_some() && needs_confirmation) {
            let Some(current_path) = self.paths.get(&coord) else {
                return None;
            };
            if *current_path != NetworkPath::Crossroad {
                return direction;
            }
            let neighbors = get_neighbors(coord, &self.grid);
            for (neighbor_direction, neighbor) in neighbors {
                let Some(p) = self.paths.get(&neighbor) else {
                    continue;
                };
                match (current_direction, neighbor_direction, p) {
                    (Direction::Down | Direction::Up, Direction::Left, NetworkPath::Horizontal) => {
                        direction = Some(Direction::Left);
                        break;
                    }
                    (
                        Direction::Down | Direction::Up,
                        Direction::Right,
                        NetworkPath::Horizontal,
                    ) => {
                        direction = Some(Direction::Right);
                        break;
                    }
                    (Direction::Left | Direction::Right, Direction::Up, NetworkPath::Vertical) => {
                        direction = Some(Direction::Up);
                        break;
                    }
                    (
                        Direction::Left | Direction::Right,
                        Direction::Down,
                        NetworkPath::Vertical,
                    ) => {
                        direction = Some(Direction::Down);
                        break;
                    }
                    _ => {}
                }
            }
        }
        direction
    }

    fn walk(&mut self) {
        let mut queue = Vec::from([(Direction::Down, self.entry)]);
        let mut visited = HashSet::from([(self.entry, Direction::Down)]);
        while !queue.is_empty() {
            let Some((direction, current)) = queue.pop() else {
                continue;
            };
            let c = self.grid[current.0][current.1];
            // println!("Visiting {} {:?} by {:?}", c, current, direction);
            if c.is_alphabetic() {
                self.letter_path += &c.to_string();
            }
            let Some(new_direction) = self.next_direction(current, direction) else {
                continue;
            };
            let neighbor = new_direction.move_by(current);
            if visited.contains(&(neighbor, new_direction)) {
                continue;
            }
            visited.insert((neighbor, new_direction));
            queue.push((new_direction, neighbor));
        }
        self.steps = visited.len();
    }
}

impl Default for Network {
    fn default() -> Self {
        Network {
            entry: (0, 0),
            steps: 0,
            grid: Vec::new(),
            letter_path: String::new(),
            paths: HashMap::new(),
        }
    }
}

fn in_bounds(coord: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
    coord.0 < grid.len() && coord.1 < grid[coord.0].len()
}

fn get_neighbors(coord: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(Direction, (usize, usize))> {
    Direction::ALL
        .iter()
        .map(|d| (*d, d.move_by(coord)))
        .filter(|(_, c)| in_bounds(*c, grid))
        .collect()
}

fn read_input(path: &str) -> Network {
    let mut network = Network::default();
    let s = fs::read_to_string(path).expect("Input file must exist");
    for (row, line) in s.lines().enumerate() {
        network.grid.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            network.grid[row].push(c);
            if row == 0 && c == '|' {
                network.entry = (row, col);
            }
            let path_result: Result<NetworkPath, &str> = c.try_into();
            match path_result {
                Ok(p) => network.paths.insert((row, col), p),
                Err(_) => None,
            };
        }
    }
    for row in 0..network.grid.len() {
        for col in 0..network.grid[row].len() {
            let char = network.grid[row][col];
            if !char.is_alphabetic() {
                continue;
            }
            let neighbors = get_neighbors((row, col), &network.grid);
            let mut ports = (false, false, false, false); // up, down, left, right
            for (direction, neighbor) in neighbors {
                let Some(p) = network.paths.get(&neighbor) else {
                    continue;
                };
                match (direction, p) {
                    (
                        Direction::Up | Direction::Down,
                        NetworkPath::Crossroad | NetworkPath::Vertical,
                    ) => {
                        ports.0 = true;
                        ports.1 = true;
                    }
                    (
                        Direction::Left | Direction::Right,
                        NetworkPath::Crossroad | NetworkPath::Horizontal,
                    ) => {
                        ports.2 = true;
                        ports.3 = true;
                    }
                    _ => {}
                }
            }
            if let Some(p) = (match ports {
                (true, true, true, true) => Some(NetworkPath::Crossroad),
                (true, true, false, false) => Some(NetworkPath::Vertical),
                (false, false, true, true) => Some(NetworkPath::Horizontal),
                _ => None,
            }) {
                network.paths.insert((row, col), p);
            }
        }
    }
    network
}

fn part_one(network: Network) -> String {
    let mut network = network;
    network.walk();
    network.letter_path
}

fn part_two(network: Network) -> usize {
    let mut network = network;
    network.walk();
    network.steps
}

fn main() {
    let network = read_input("input.txt");
    println!("{:?}", network.entry);
    println!("Part one: {}", part_one(network.clone()));
    println!("Part two: {}", part_two(network));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let network = read_input("test.txt");
        assert_eq!(&part_one(network), "ABCDEF")
    }

    #[test]
    fn test_part_two() {
        let network = read_input("test.txt");
        assert_eq!(part_two(network), 38)
    }
}
