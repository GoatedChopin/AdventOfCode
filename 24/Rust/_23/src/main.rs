use std::fs;

fn read_input(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents.lines().map(|line| line.to_string()).collect()
}

fn main() {
    println!("Hello, world!");
}
