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
    let mut can_merge = true;
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.min.cmp(&b.min));
    while can_merge {
        let mut removable = Vec::new();
        let mut combinable = Vec::new();
        'outer: for a in 0..sorted_ranges.len() {
            for b in 0..sorted_ranges.len() {
                if a == b {
                    continue;
                }
                let range_a = &sorted_ranges[a];
                let range_b = &sorted_ranges[b];
                // println!(
                //     "Checking if {} {:?} is contained in {} {:?}",
                //     a, range_a, b, range_b
                // );
                if range_a.min >= range_b.min && range_a.max <= range_b.max {
                    // println!(
                    //     "Adding removable: {} {:?}, which is contained in {} {:?}",
                    //     a, range_a, b, range_b
                    // );
                    removable.push(a);
                    continue 'outer;
                } else if range_b.min >= range_a.min && range_b.max <= range_a.max {
                    removable.push(b);
                    continue 'outer;
                } else if range_a.min <= range_b.max && range_a.max >= range_b.min {
                    for (i, j) in &combinable {
                        if (*i == a || *j == a) || (*i == b || *j == b) {
                            // println!("Skipping combinable, one of the indexes is scheduled for combination: {:?} and {:?}", range_a, range_b);
                            continue 'outer;
                        }
                    }
                    // println!("Adding combinable: {:?} and {:?}", range_a, range_b);
                    combinable.push((a, b));
                    continue 'outer;
                }
            }
        }
        if removable.is_empty() && combinable.is_empty() {
            can_merge = false;
        }

        let mut new_sorted_ranges = Vec::new();
        'outer: for i in 0..sorted_ranges.len() {
            if removable.contains(&i) {
                continue;
            }
            for (a, b) in combinable.iter() {
                if a == &i {
                    let new_min = if sorted_ranges[*a].min < sorted_ranges[*b].min { sorted_ranges[*a].min } else { sorted_ranges[*b].min };
                    let new_max = if sorted_ranges[*a].max < sorted_ranges[*b].max { sorted_ranges[*b].max } else { sorted_ranges[*a].max };
                    new_sorted_ranges.push(Range {
                        min: new_min,
                        max: new_max,
                    });
                    continue 'outer;
                } else if b == &i {
                  // `b` is redundant after combination with `a`
                  continue 'outer;
                }
            }
            new_sorted_ranges.push(sorted_ranges[i].clone());
        }
        sorted_ranges = new_sorted_ranges;
    }

    let mut final_pass = sorted_ranges.clone();
    final_pass.sort_by(|a, b| a.min.cmp(&b.min));
    for i in 0..final_pass.len() - 1 {
        let current = &final_pass[i];
        let next = &final_pass[i + 1];
        assert!(current.min <= current.max);
        assert!(next.min <= next.max);
        assert!(current.max < next.min);
    }

    let mut sum = 0;
    for range in sorted_ranges.iter() {
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
