use std::{collections::{HashMap, HashSet}, fs};

struct Problem {
    patterns: HashSet<String>,
    designs: Vec<String>,
}

fn read_input(file_path: &str) -> Problem {
    let lines = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = lines.split("\n").collect::<Vec<&str>>();
    let patterns = lines[0]
        .replace(" ", "")
        .split(",")
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();
    let designs = lines[2..]
        .to_vec()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    Problem {
        patterns: patterns,
        designs: designs,
    }
}

fn can_make_design(design: &str, patterns: &HashSet<String>) -> bool {
    print!("{}: ", design);
    if design.is_empty() || patterns.contains(design) {
        return true;
    }
    let mut iterations = 0;
    let max_pattern_length = patterns.iter().map(|pattern| pattern.len()).max().unwrap();
    let max_slice_length = std::cmp::min(max_pattern_length, design.len());
    // let chars = design.chars().collect::<Vec<char>>();
    let mut can_reach = vec![false; design.len() + 1];
    let mut visited = vec![false; design.len() + 1];
    let mut last_reachable = 0;
    while last_reachable < design.len() && iterations < 1000000 {
      iterations += 1;
      let mut can_reach_next = false;
      let mut max_reachable = last_reachable;
      for i in (last_reachable + 1)..=std::cmp::min(last_reachable + max_slice_length, design.len()) {
        let slice = &design[last_reachable..i];
        if patterns.contains(slice) {
          can_reach[i] = true;
          if !visited[i] {
            can_reach_next = true;
            max_reachable = i;
          }
        }
      }
      last_reachable = max_reachable;
      visited[last_reachable] = true;
      if !can_reach_next {
        // println!("Reached a dead end at index {}", last_reachable);
        // println!("{}", can_reach.iter().enumerate().map(|(i, _)| if i == last_reachable { "v" } else { " " }).collect::<Vec<&str>>().join(""));
        // println!("{}", design[..last_reachable].to_string());
        // println!("{}", can_reach.iter().map(|b| if *b { "1" } else { "0" }).collect::<Vec<&str>>().join(""));
        // println!("{}", visited.iter().map(|b| if *b { "1" } else { "0" }).collect::<Vec<&str>>().join(""));
        while last_reachable > 0 && (!can_reach[last_reachable] || visited[last_reachable]) {
          last_reachable -= 1;
        }
        if last_reachable == 0 {
          return false;
        }
        visited[last_reachable] = true;
      }
    } 
    if iterations >= 1000000 {
      println!();
      println!("{}", patterns.iter().map(|p| p.as_str()).collect::<Vec<&str>>().join(", "));
      println!();
      println!("{}", design);
      println!();
    }
    true
}

fn part_one(problem: &Problem) -> usize {
    problem
        .designs
        .iter()
        .filter(|design| {
          let result = can_make_design(design.as_str(), &problem.patterns);
          println!("{}", result);
          result
          })
        .count()
}

fn num_options(design: &str, patterns: &HashSet<String>, max_pattern_length: usize, memo: &mut HashMap<String, usize>) -> usize {
    if memo.contains_key(design) {
        return memo[design];
    }
    if design.is_empty() {
        return 1;
    }
    let mut options = 0;
    for i in (1..=std::cmp::min(max_pattern_length, design.len())).rev() {
        let slice = &design[..i];
        if patterns.contains(slice) {
            options += num_options(&design[i..], patterns, max_pattern_length, memo);
        }
    }
    memo.insert(design.to_string(), options);
    memo[design]
}

fn part_two(problem: &Problem) -> usize {
    let max_pattern_length = problem
        .patterns
        .iter()
        .map(|pattern| pattern.len())
        .max()
        .unwrap();
    problem
        .designs
        .iter()
        .map(|design| {
          let result = num_options(design.as_str(), &problem.patterns, max_pattern_length, &mut HashMap::new());
          println!("{}: {}", design, result);
          result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
      brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
      bggr can be made with a b towel, two g towels, and then an r towel.
      gbbr can be made with a gb towel and then a br towel.
      rrbgbr can be made with r, rb, g, and br.
      ubwu is impossible.
      bwurrg can be made with bwu, r, r, and g.
      brgr can be made with br, g, and r.
      bbrgwb is impossible.
    */

    #[test]
    fn test_can_make_design() {
        let problem = read_input("test.txt");

        // possible
        assert!(can_make_design("brwrr", &problem.patterns));
        assert!(can_make_design("bggr", &problem.patterns));
        assert!(can_make_design("gbbr", &problem.patterns));
        assert!(can_make_design("rrbgbr", &problem.patterns));
        assert!(can_make_design("bwurrg", &problem.patterns));
        assert!(can_make_design("brgr", &problem.patterns));

        // impossible
        assert_eq!(
            can_make_design("ubwu", &problem.patterns),
            false
        );
        assert_eq!(
            can_make_design("bbrgwb", &problem.patterns),
            false
        );
    }

    #[test]
    fn test_part_one() {
        let problem = read_input("test.txt");
        assert_eq!(part_one(&problem), 6);
    }

    #[test]
    fn test_part_one_infinite() {
        let problem = read_input("infinite.txt");
        assert_eq!(part_one(&problem), 0);
    }

    #[test]
    fn test_part_two() {
        let problem = read_input("test.txt");
        assert_eq!(part_two(&problem), 16);
    }
}

fn main() {
    let problem = read_input("input.txt");
    println!("Part one: {}", part_one(&problem));
    println!("Part two: {}", part_two(&problem));
}
