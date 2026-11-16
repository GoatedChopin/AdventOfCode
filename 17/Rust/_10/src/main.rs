use std::collections::HashSet;
use std::fs;

fn read_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn get_index(i: usize, list: &Vec<usize>) -> usize {
    i % list.len()
}

fn twist(list: &mut Vec<usize>, length: usize, current_position: usize) {
    for i in 0..length / 2 {
        let left = (current_position + i) % list.len();
        let right = (current_position + length - 1 - i) % list.len();

        list.swap(left, right);
    }
}

fn part_one(input: &[usize], ring_size: usize) -> usize {
    let mut list = (0..=ring_size).collect::<Vec<usize>>();
    let mut current = 0;
    let mut skip = 0;
    for length in input {
        twist(&mut list, *length, current);
        current = get_index(current + length + skip, &list);
        skip += 1;
    }
    list[0] * list[1]
}

fn convert_to_ascii(input: &str) -> Vec<u8> {
    input.bytes().collect()
}

fn compute_dense_hashes(list: &Vec<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for i in (0..list.len()).step_by(16) {
        let mut dense = list[i];
        for j in i + 1..i + 16 {
            dense ^= list[j];
        }
        out.push(dense);
    }
    out
}

fn part_two(input: &Vec<u8>) -> String {
    let mut input = input.clone();
    input.extend([17, 31, 73, 47, 23].iter());
    let input: Vec<usize> = input.into_iter().map(|i| i.into()).collect();
    let mut list = (0..=255).collect::<Vec<usize>>();
    let mut current = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for length in input.iter() {
            twist(&mut list, *length, current);
            current = get_index(current + length + skip, &list);
            skip += 1;
        }
    }
    let dense_hashes = compute_dense_hashes(&list);

    dense_hashes.iter().map(|d| format!("{:02x}", d)).collect()
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input, 255));
    let input = convert_to_ascii(
        fs::read_to_string("input.txt")
            .expect("Input file must exist")
            .trim(),
    );
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        let outcomes: [[usize; 5]; 4] = [
            [2, 1, 0, 3, 4],
            [4, 3, 0, 1, 2],
            [4, 3, 0, 1, 2],
            [3, 4, 2, 1, 0],
        ];

        let mut list = (0..=4).collect::<Vec<usize>>();
        let mut current = 0;
        let mut skip = 0;
        for (i, length) in input.iter().enumerate() {
            println!("Twisting {} from {} with skip {}", *length, current, skip);
            twist(&mut list, *length, current);
            for (mi, mu) in outcomes[i].iter().enumerate() {
                println!("Expected\t{:?}\nGot\t\t{:?}\n\n", outcomes[i], list);
                assert_eq!(*mu, list[mi]);
            }
            current = get_index(current + length + skip, &list);
            skip += 1;
        }
        assert_eq!(list[0] * list[1], 12);

        assert_eq!(part_one(&input, 4), 12);
    }

    #[test]
    fn test_to_ascii() {
        let v = convert_to_ascii("1,2,3");
        assert_eq!(v, Vec::from([49, 44, 50, 44, 51]));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&convert_to_ascii("")),
            "a2582a3a0e66e6e86e3812dcb672a272".to_string()
        );

        assert_eq!(
            part_two(&convert_to_ascii("AoC 2017")),
            "33efeb34ea91902bb2f59c9920caa6cd".to_string()
        );

        assert_eq!(
            part_two(&convert_to_ascii("1,2,3")),
            "3efbe78a8d82f29979031a4aa0b16a9d".to_string()
        );

        assert_eq!(
            part_two(&convert_to_ascii("1,2,4")),
            "63960835bcdc130f0b66d7ff4f6a5a8e".to_string()
        );
    }
}
