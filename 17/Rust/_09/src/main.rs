use std::fs;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_string()
}

fn part_one(input: &str) -> usize {
    let mut score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut cancel_next = false;
    for c in input.chars() {
        if cancel_next {
          cancel_next = false;
          continue;
        }
        if c == '!' {
          cancel_next = true;
          continue;
        }
        if in_garbage {
          if c == '>' {
            in_garbage = false;
          }
          continue;
        }
        if c == '<' {
          in_garbage = true;
        }
        if c == '{' {
          depth += 1;
        }
        if c == '}' {
          score += depth;
          depth -= 1;
        }
    }
    score
}

fn part_two(input: &str) -> usize {
    let mut garbage_chars = 0;
    let mut in_garbage = false;
    let mut cancel_next = false;
    for c in input.chars() {
        if cancel_next {
          cancel_next = false;
          continue;
        }
        if c == '!' {
          cancel_next = true;
          continue;
        }
        if in_garbage {
          if c == '>' {
            in_garbage = false;
          } else {
            garbage_chars += 1;
          }
          continue;
        }
        if c == '<' {
          in_garbage = true;
        }
    }
    garbage_chars
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
