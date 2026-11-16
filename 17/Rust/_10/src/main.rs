use std::collections::{HashSet, VecDeque};
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

fn get_index(i: usize, list: &VecDeque<usize>) -> usize {
    if i >= list.len() { i % list.len() } else { i }
}

fn twist(list: &mut VecDeque<usize>, length: usize, current_position: usize) {
    let mut visited = HashSet::new();
    for i in current_position..current_position + (length.saturating_sub(2)) {
        let left = get_index(i, &list);
        let right = get_index(
            current_position + length - 1 - (i - current_position),
            &list,
        );
        if visited.contains(&left) || visited.contains(&right) {
            continue;
        }
        visited.insert(left);
        visited.insert(right);
        println!("Swapping {} <-> {}", left, right);
        list.swap(left, right);
        println!("{:?}", list);
    }
}

fn part_one(input: &Vec<usize>, ring_size: usize) -> usize {
    let mut list = (0..=ring_size).collect::<VecDeque<usize>>();
    let mut current = 0;
    let mut skip = 0;
    for length in input {
        twist(&mut list, *length, current);
        current = get_index(current + length + skip, &list);
        skip += 1;
    }
    list[0] * list[1]
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input, 255));
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

        let mut list = (0..=4).collect::<VecDeque<usize>>();
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
}
