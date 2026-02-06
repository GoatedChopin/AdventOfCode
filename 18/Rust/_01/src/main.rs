use std::{collections::HashSet, fs};

fn read_input(path: &str) -> Vec<isize> {
    fs::read_to_string(path).expect("Failed to read input file")
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

fn part_one(input: &[isize]) -> isize {
    input.iter().sum()
}

fn part_two(input: &[isize]) -> isize {
    let mut seen = HashSet::new();
    let mut index = 0;
    let mut current = 0;
    seen.insert(current);
    loop {
      current += input[index];
      if seen.contains(&current) {
        return current;
      }
      seen.insert(current);
      index = (index + 1) % input.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = read_input("test1.txt");
        assert_eq!(part_two(&input), 0);
        let input = read_input("test2.txt");
        assert_eq!(part_two(&input), 10);
    }

    #[test]
    fn test_part_one() {
        let input = read_input("test1.txt");
        assert_eq!(part_one(&input), 0);
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
