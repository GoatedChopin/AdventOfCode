use std::fs;

struct Rotation {
    direction: i32,
    steps: i32,
}

fn read_input(path: &str) -> Vec<Rotation> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| {
            let direction = if line.contains("R") { 1 } else { -1 };
            let steps = line
                .replace("R", "")
                .replace("L", "")
                .parse::<i32>()
                .unwrap();
            Rotation { direction, steps }
        })
        .collect::<Vec<Rotation>>()
}

fn part_one(input: &Vec<Rotation>) -> i32 {
    let mut visits_to_zero = 0;
    let mut current_position = 50;
    for rotation in input {
        current_position += rotation.direction * rotation.steps;
        while current_position < 0 {
            current_position += 100;
        }
        while current_position >= 100 {
            current_position -= 100;
        }
        if current_position == 0 {
            visits_to_zero += 1;
        }
    }
    visits_to_zero
}

fn part_two(input: &Vec<Rotation>) -> i32 {
    let mut visits_to_zero = 0;
    let mut current_position = 50;
    for rotation in input {
        let mut steps_taken = 0;
        while steps_taken < rotation.steps {
            current_position += rotation.direction;
            steps_taken += 1;
            while current_position < 0 {
                current_position += 100;
            }
            while current_position >= 100 {
                current_position -= 100;
            }
            if current_position == 0 {
                visits_to_zero += 1;
            }
        }
    }
    visits_to_zero
}

fn main() {
    let test_input = read_input("test.txt");
    assert_eq!(part_one(&test_input), 3);
    assert_eq!(part_two(&test_input), 6);
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
