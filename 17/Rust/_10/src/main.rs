use std::fs;

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn part_one(input: &Vec<i32>) -> i32 {
    let mut list = (0..256).collect::<Vec<i32>>();
    let mut current = 0;
    let mut skip = 0;
    for length in input {
        list = twist(list, *length, current);
    }
    list[0] * list[1]
}

fn main() {
    println!("Hello, world!");
}
