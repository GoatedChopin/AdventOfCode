use std::fs;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

fn read_input(file_path: &str) -> Vec<Coordinate> {
    let mut coordinates = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
          let parts: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
          Coordinate { row: parts[0], col: parts[1] }
        })
        .collect::<Vec<Coordinate>>();

    coordinates.sort_by(|a, b| a.row.cmp(&b.row).then(a.col.cmp(&b.col)));

    coordinates
}

fn calculate_area(left: Coordinate, right: Coordinate) -> usize {
    let right_distance = if right.col > left.col {
        (right.col - left.col) + 1
    } else {
        (left.col - right.col) + 1
    };
    let left_distance = if left.row > right.row {
        (left.row - right.row) + 1
    } else {
        (right.row - left.row) + 1
    };
    right_distance * left_distance
}

fn part_one(coordinates: &Vec<Coordinate>) -> usize {
    let mut max_area = 0;
    for i in 0..((coordinates.len() / 2) + 1) {
      let left_index = i;
      let mut right_index = coordinates.len() - 1 - i;
        while left_index < right_index {
            let left = coordinates[left_index];
            let right = coordinates[right_index];
            let area = calculate_area(left, right);
            // println!("left: {:?}, right: {:?}, area: {}", left, right, area);
            if area > max_area {
                println!("New best: {:?} & {:?} -> {}", left, right, area);
                max_area = area;
            }
            right_index -= 1;
        }
    }
    max_area
}

fn test() {
    let coordinates = read_input("test.txt");
    assert_eq!(part_one(&coordinates), 50);
}

fn main() {
    test();
    let coordinates = read_input("input.txt");
    println!("{}", part_one(&coordinates));
}
