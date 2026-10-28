use std::fs;

fn read_input(path: &str) -> Vec<usize> {
    let i = fs::read_to_string(path).expect("Input file must exist");
    i.lines()
        .into_iter()
        .map(|l| l.parse().expect("Bad input usize"))
        .collect()
}

fn main() {
    println!("Hello, world!");
}
