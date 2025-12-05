use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Range {
    min: u64,
    max: u64,
}

fn read_input(path: &str) -> (Vec<Range>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut numbers = Vec::new();
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .for_each(|line| {
            if line.contains("-") {
                let mut parts = line
                    .split("-")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                parts.sort();
                ranges.push(Range {
                    min: parts[0],
                    max: parts[1],
                });
            } else if line.trim().is_empty() {
                return;
            } else {
                numbers.push(line.parse::<u64>().unwrap());
            }
        });
    (ranges, numbers)
}

fn part_one(ranges: &Vec<Range>, numbers: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for number in numbers {
        for range in ranges {
            if *number >= range.min && *number <= range.max {
                sum += 1;
                break;
            }
        }
    }
    sum
}

fn part_two(ranges: &Vec<Range>) -> u64 {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut stack = Vec::new();
    stack.push(sorted_ranges[0]);
    for i in 1..sorted_ranges.len() {
        let current = sorted_ranges[i];
        let last = stack.last().unwrap().clone();
        if current.min <= last.max + 1 {
            let new_max = if current.max > last.max { current.max } else { last.max };
            stack.pop();
            stack.push(Range { min: last.min, max: new_max });
            continue;
        } else {
            stack.push(current);
        }
    }

    let mut sum = 0;
    for range in stack.iter() {
        sum += (range.max - range.min) + 1;
    }
    sum
}

fn test() {
    let input = read_input("test.txt");
    assert_eq!(part_one(&input.0, &input.1), 3);
    assert_eq!(part_two(&input.0), 14);
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input.0, &input.1));
    println!("Part two: {}", part_two(&input.0));
}
