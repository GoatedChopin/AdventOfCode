use std::fs;

/*
Shoutout to NoSpawnn for posting their solution at
https://github.com/NoSpawnn/advent_of_code_2025/blob/main/src/09/main.rs
It was much more efficient than my first pass and has better Rust ergonomics
*/

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        let (col, row) = value.split_once(',').unwrap();
        Self {
            row: row.parse().expect("row should be numeric"),
            col: col.parse().expect("col should be numeric"),
        }
    }
}

fn read_input(file_path: &str) -> Vec<Coordinate> {
    let coordinates = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(Coordinate::from)
        .collect::<Vec<Coordinate>>();

    coordinates
}

fn calculate_area(left: Coordinate, right: Coordinate) -> usize {
    let width = left.row.abs_diff(right.row) + 1;
    let height = left.col.abs_diff(right.col) + 1;
    width * height
}

fn part_one(coordinates: &Vec<Coordinate>) -> usize {
    let mut max_area = 0;
    for left_index in 0..coordinates.len() {
        let mut right_index = coordinates.len() - 1;
        while left_index < right_index {
            let left = coordinates[left_index];
            let right = coordinates[right_index];
            let area = calculate_area(left, right);
            // println!("left: {:?}, right: {:?}, area: {}", left, right, area);
            if area > max_area {
                // println!("New best: {:?} & {:?} -> {}", left, right, area);
                max_area = area;
            }
            right_index -= 1;
        }
    }
    max_area
}

fn inside_all_sides(
    c1: &Coordinate,
    c2: &Coordinate,
    sides: &Vec<(&Coordinate, &Coordinate)>,
) -> bool {
    sides.iter().all(|(start, end)| {
        let before = c1.col.max(c2.col) <= start.col.min(end.col);
        let after = c1.col.min(c2.col) >= start.col.max(end.col);
        let above = c1.row.max(c2.row) <= start.row.min(end.row);
        let below = c1.row.min(c2.row) >= start.row.max(end.row);
        before || after || above || below
    })
}

fn part_two(coordinates: &Vec<Coordinate>) -> usize {
    let sides: Vec<_> = coordinates
        .windows(2)
        .map(|coords| (&coords[0], &coords[1]))
        .chain([(&coordinates[coordinates.len() - 1], &coordinates[0])])
        .collect();

    let mut possible_rects: Vec<_> = coordinates
        .iter()
        .enumerate()
        .flat_map(|(i, c1)| {
            coordinates[i + 1..]
                .iter()
                .map(move |c2| (c1, c2, calculate_area(*c1, *c2)))
        })
        .collect();

    possible_rects.sort_by_key(|(_, _, a)| *a);

    possible_rects
        .into_iter()
        .rev()
        .find(|(c1, c2, _)| inside_all_sides(&c1, &c2, &sides))
        .expect("possible should not be empty")
        .2
}

fn test() {
    let coordinates = read_input("test.txt");
    assert_eq!(part_one(&coordinates), 50);
    assert_eq!(part_two(&coordinates), 24);
}

fn main() {
    test();
    let coordinates = read_input("input.txt");
    println!("{}", part_one(&coordinates));
    println!("{}", part_two(&coordinates));
}
