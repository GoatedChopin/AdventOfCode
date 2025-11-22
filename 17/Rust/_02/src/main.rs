use std::fs;

fn read_input(path: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|line| line.split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in input {
        let mut max: u32 = 0;
        let mut min: u32 = u32::MAX;
        for num in row {
            if *num > max {
                max = *num;
            }
            if *num < min {
                min = *num;
            }
        }
        sum += max - min;
    }
    sum as u32
}

fn find_divisible(row: &Vec<u32>) -> u32 {
    for i in 0..row.len() {
        for j in 0..row.len() {
            if i != j && row[i] % row[j] == 0 {
                return row[i] / row[j];
            }
        }
    }
    return 0;
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in input {
        sum += find_divisible(row);
    }
    sum as u32
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
