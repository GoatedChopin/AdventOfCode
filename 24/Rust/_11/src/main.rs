use std::fs;
use std::collections::HashMap;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

fn split_stone(stone: &str) -> (String, String) {
    let len = stone.len();
    let mid = len / 2;
    let left = stone[..mid].to_string();
    let mut right = stone[mid..].to_string();
    while right.len() > 1 && right.starts_with('0') {
        right = right[1..].to_string();
    }
    (left, right)
}

fn multiply_stone(stone: &str) -> String {
    let num = stone.parse::<i64>().expect("Failed to parse stone");
    (num * 2024).to_string()
}

fn process_stone(stone: String) -> Vec<String> {
    if stone == "0" {
        return vec!["1".to_string()];
    } else if stone.len() % 2 == 0 {
        let (left, right) = split_stone(&stone);
        if right.is_empty() {
          return vec![left];
        }
        return vec![left, right];
    }
    vec![multiply_stone(&stone)]
}

fn step(stones: Vec<String>) -> Vec<String> {
    stones.into_iter().flat_map(|stone| process_stone(stone)).collect()
}

fn part_one(stones: Vec<String>, steps: usize) -> i64 {
  let mut new_stones = stones;
  for _i in 0..steps {
    new_stones = step(new_stones);
  }
  new_stones.len() as i64
}

fn count_stones_memo(stone: &str, steps: usize, cache: &mut HashMap<(String, usize), i64>) -> i64 {
    if steps == 0 {
        return 1;
    }
    
    let key = (stone.to_string(), steps);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }
    
    let result = if stone == "0" {
        count_stones_memo("1", steps - 1, cache)
    } else if stone.len() % 2 == 0 {
        let (left, right) = split_stone(stone);
        let left_count = count_stones_memo(&left, steps - 1, cache);
        let right_str = if right.is_empty() { "0" } else { &right };
        let right_count = count_stones_memo(right_str, steps - 1, cache);
        left_count + right_count
    } else {
        let new_stone = multiply_stone(stone);
        count_stones_memo(&new_stone, steps - 1, cache)
    };
    
    cache.insert(key, result);
    result
}

fn part_two(stones: Vec<String>, steps: usize) -> i64 {
    let mut cache = HashMap::new();
    stones.iter()
        .map(|stone| count_stones_memo(stone, steps, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let stones: Vec<String> = read_file("test.txt").split_whitespace().map(|s| s.to_string()).collect();
        let result = part_one(stones.clone(), 6);
        assert_eq!(result, 22);
        let result = part_one(stones.clone(), 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_two() {
        let stones: Vec<String> = read_file("test.txt").split_whitespace().map(|s| s.to_string()).collect();
        let result = part_two(stones.clone(), 6);
        assert_eq!(result, 22);
        let result = part_two(stones.clone(), 25);
        assert_eq!(result, 55312);
        let result = part_two(stones.clone(), 75);
        println!("Part 2 (75 steps): {}", result);
    }

    #[test]
    fn test_zero() {
      let stones = vec!["0".to_string()];
      let result = part_two(stones, 75);
      println!("Single zero after 75 steps: {}", result);
    }
}

fn main() {
    let input = read_file("input.txt");
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
    // Part 1: 25 steps
    let result = part_one(stones.clone(), 25);
    println!("Part 1 (25 steps): {}", result);
    
    // Part 2: 75 steps
    let result = part_two(stones.clone(), 75);
    println!("Part 2 (75 steps): {}", result);
}
