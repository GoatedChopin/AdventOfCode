use std::fs;
use std::path::Path;

fn read_input<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed to read input file").trim().to_string()
}

fn part_one(secret: &str) -> u32 {
    find_number(secret, 5)
}

fn part_two(secret: &str) -> u32 {
    find_number(secret, 6)
}

fn find_number(secret: &str, zeros: u8) -> u32 {
    let mut i: u32 = 1;
    loop {
        let s = format!("{}{}", secret, i);
        let digest = md5::compute(s.as_bytes());
        let d = digest.0;
        let ok = if zeros == 5 {
            d[0] == 0 && d[1] == 0 && (d[2] & 0xF0) == 0
        } else {
            d[0] == 0 && d[1] == 0 && d[2] == 0
        };
        if ok {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let input_path = "./input.txt";
    let secret = read_input(input_path);
    let p1 = part_one(&secret);
    let p2 = part_two(&secret);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_examples() {
        assert_eq!(part_one("abcdef"), 609043);
        assert_eq!(part_one("pqrstuv"), 1048970);
    }

    #[test]
    fn test_part_two_examples() {
        assert_eq!(part_two("abcdef"), 6742839);
        assert_eq!(part_two("pqrstuv"), 5714438);
    }
}
