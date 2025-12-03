use std::fs;

struct Range {
    start: u64,
    end: u64,
}

fn read_input(path: &str) -> Vec<Range> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_string()
        .split(",")
        .map(|range| {
            let parts = range.split("-").collect::<Vec<&str>>();
            Range {
                start: parts[0].parse::<u64>().unwrap(),
                end: parts[1].parse::<u64>().unwrap(),
            }
        })
        .collect::<Vec<Range>>()
}

fn is_equal_halves(id: u64) -> bool {
    let chars = id.to_string().chars().collect::<Vec<char>>();

    if chars.len() % 2 != 0 {
        return false;
    }

    let mut left = 0;
    let mut right = chars.len() / 2;
    // println!("Starting at indexes: {} and {}", left, right);
    while right < chars.len() {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right += 1;
    }
    true
}

fn part_one(input: &Vec<Range>, noisy: bool) -> u64 {
    let mut sum = 0;
    for range in input {
        for id in range.start..range.end + 1 {
            if is_equal_halves(id) {
                if noisy {
                    println!("Found repeating chars: {}", id);
                }
                sum += id;
            }
        }
    }
    sum
}

fn get_starting_slices(chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut slices = Vec::new();
    for i in 0..(chars.len() / 2) {
        if chars.len() % (i + 1) != 0 {
            continue;
        }
        slices.push(chars[..i + 1].to_vec());
    }
    slices
}

fn is_repeating_chars(id: u64) -> bool {
    let chars = id.to_string().chars().collect::<Vec<char>>();

    // Find all slices of the start of the string that evenly fit into the string (1, 2 for even, 3 for multiples of 3, etc.)
    // ...all the way up to half of the length of the string, if the string is an even length.

    // Use an inner pointer and an outer pointer to see if the ID is just the slice repeated N times.

    let slices = get_starting_slices(&chars);

    'slice_loop: for slice in slices {
        let mut inner = 0;
        for outer in 0..chars.len() {
            if chars[outer] != chars[inner] {
                continue 'slice_loop;
            }
            inner += 1;
            if inner == slice.len() {
                inner = 0;
            }
        }
        return true;
    }
    return false;
}

fn part_two(input: &Vec<Range>, noisy: bool) -> u64 {
    let mut sum = 0;
    for range in input {
        for id in range.start..range.end + 1 {
            if is_repeating_chars(id) {
                if noisy {
                    println!("Found repeating chars: {}", id);
                }
                sum += id;
            }
        }
    }
    sum
}

fn test() {
    // Part one
    let positives = vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859];
    for positive in positives {
        assert!(is_equal_halves(positive));
    }
    let negatives = vec![
        12, 95, 98, 115, 1188511880, 222221, 446443, 38593856, 565653, 824824821,
    ];
    for negative in negatives {
        assert!(!is_equal_halves(negative));
    }
    let input = read_input("test.txt");
    assert_eq!(
        part_one(&input, false),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );

    // Part two
    let repeating = vec![1188511885, 121212, 123123123];
    for repeating in repeating {
        assert!(is_repeating_chars(repeating));
    }
    assert_eq!(part_two(&input, false), 4174379265);

    println!("Tests passed");
}

fn main() {
    test();
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(&input, false));
    println!("Part two: {}", part_two(&input, false));
}
