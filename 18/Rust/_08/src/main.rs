use std::fs;

#[derive(Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
    length: usize,
}

impl Node {
    fn from_vec(v: &Vec<usize>, start: Option<usize>) -> Self {
        let mut current_position = start.unwrap_or(0);
        let mut children = Vec::new();
        let mut metadata = Vec::new();

        let num_children = v[current_position];
        let num_metadata = v[current_position + 1];

        current_position += 2;
        while children.len() < num_children && current_position < v.len() {
            let child = Node::from_vec(v, Some(current_position));
            current_position += child.length;
            children.push(child);
        }

        let children_total_length = children.iter().map(|c| c.length).sum::<usize>();

        let metadata_position = start.unwrap_or(0) + children_total_length + 2;
        for i in metadata_position..(metadata_position + num_metadata) {
            metadata.push(v[i]);
        }

        let length = 2 + num_metadata + children_total_length;

        Self {
            children,
            metadata,
            length,
        }
    }

    fn value(&self) -> usize {
        match self.children.len() == 0 {
            true => self.metadata.iter().sum::<usize>(),
            false => self
                .metadata
                .iter()
                .map(|m| {
                    if *m > self.children.len() || *m == 0 {
                        return 0;
                    }
                    return self.children[m - 1].value();
                })
                .sum::<usize>(),
        }
    }
}

struct LicenseTree {
    root: Node,
}

fn read_input(file_path: &str) -> Node {
    let s = fs::read_to_string(file_path).expect("Bad file path");
    let v = s
        .split_whitespace()
        .map(|p| p.parse::<usize>().unwrap())
        .collect();
    let root = Node::from_vec(&v, None);
    root
}

fn part_one(lt: Node) -> usize {
    let mut total = 0;

    total += lt.metadata.iter().sum::<usize>();
    for child in lt.children {
        total += part_one(child);
    }

    total
}

fn part_two(lt: Node) -> usize {
    lt.value()
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 138);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("test.txt");
        assert_eq!(part_two(input), 66)
    }
}
