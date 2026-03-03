use std::fs;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn part_one(input: &str) -> i32 {
    input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(1),
            ')' => Some(-1),
            _ => None, // ignore newlines/whitespace
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return idx + 1; // 1-indexed position
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("()()"), 0);
        assert_eq!(part_one("((("), 3);
        assert_eq!(part_one("(()(()("), 3);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one("())"), -1);
        assert_eq!(part_one("))("), -1);
        assert_eq!(part_one(")))"), -3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn examples_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
