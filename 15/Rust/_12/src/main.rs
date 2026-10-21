use serde_json::Value;

use std::fs;

fn simple_recursive_addition(parsed: &Value) -> i64 {
    if let Some(number) = parsed.as_number() {
        if let Some(i) = number.as_i64() {
            return i;
        }
    }
    let mut total = 0;
    if let Some(obj) = parsed.as_object() {
        for (_, value) in obj.iter() {
            total += simple_recursive_addition(&value);
        }
    }
    if let Some(obj) = parsed.as_array() {
        return obj.iter().map(|o| simple_recursive_addition(o)).sum();
    }
    total
}

fn red_recursive_addition(parsed: &Value) -> i64 {
    if let Some(number) = parsed.as_number() {
        if let Some(i) = number.as_i64() {
            return i;
        }
    }
    let mut total = 0;
    if let Some(obj) = parsed.as_object() {
        for (_, value) in obj.iter() {
            if value.as_str() == Some("red") {
                return 0;
            }
            total += red_recursive_addition(&value);
        }
    }
    if let Some(obj) = parsed.as_array() {
        return obj.iter().map(|o| red_recursive_addition(o)).sum();
    }
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("input.txt").expect("Bad parse");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    Ok(())
}

fn read_input(path: &str) -> Result<Value, serde_json::Error> {
    let string = fs::read_to_string(path).expect("Input file must exist");
    let parsed = serde_json::from_str(string.trim());
    parsed
}

fn part_one(parsed: &Value) -> i64 {
    simple_recursive_addition(parsed)
}

fn part_two(parsed: &Value) -> i64 {
    red_recursive_addition(parsed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let parsed: Value = serde_json::from_str("[[[3]]]").expect("Bad parse");
        assert_eq!(simple_recursive_addition(&parsed), 3);
    }
}
