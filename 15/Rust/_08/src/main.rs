use std::fs;

fn read_inputs(inputs: &str) -> Vec<String> {
    let bytes = fs::read_to_string(inputs).expect("Inputs must exist");
    bytes.lines().map(|s| s.trim().into()).collect()
}

fn is_escape(c: char) -> bool {
    c == '\\'
}

fn is_special(c: char) -> bool {
    c == '\\' || c == '"'
}

fn sizes(s: &String) -> (usize, usize) {
    let raw = s.len();
    let mut escaped_size = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut escaped = false;
    while i < chars.len() - 2 {
        i += 1;
        let c = chars[i];
        if is_escape(c) && !escaped {
            escaped = true;
        }
        if escaped && chars[i + 1] == 'x' {
            escaped = false;
            i += 3;
        }
        if escaped {
            escaped = false;
            i += 1;
        }
        escaped_size += 1;
    }
    (raw, escaped_size)
}

fn encode(s: &String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut e = Vec::new();
    e.push('"');
    for c in chars {
        if is_special(c) {
            e.push('\\');
        }
        e.push(c);
    }
    e.push('"');
    e.into_iter().collect()
}

fn part_one(lines: Vec<String>) -> usize {
    lines.iter().map(|l| {
        let (raw, escaped) = sizes(l);
        raw - escaped
    }).sum()
}

fn part_two(lines: Vec<String>) -> usize {
    lines.iter().map(|l| encode(l)).map(|l| {
        let (raw, escaped) = sizes(&l);
        raw - escaped
    }).sum()
}

fn main() {
    let lines = read_inputs("input.txt");
    println!("Part one: {}", part_one(lines.clone()));
    println!("Part two: {}", part_two(lines));
}
