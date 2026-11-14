use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Eq, PartialEq)]
struct Guess {
    steps: usize,
    inner: String,
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.inner.len() + self.steps).cmp(&(other.inner.len() + other.steps)))
    }
}

impl Ord for Guess {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.inner.len() + self.steps).cmp(&(other.inner.len() + other.steps))
    }
}

#[derive(Clone, Debug)]
struct Molecule {
    rules: HashMap<String, String>,
    inner: String,
    target: String,
}

impl Molecule {
    fn replacements(&self) -> Vec<(isize, usize, String, String)> {
        let mut r = Vec::new();

        for (before, after) in self.rules.iter() {
            let delta = after.len() as isize - before.len() as isize;

            let m = matches(&self.inner, before);

            r.extend(
                m.into_iter()
                    .map(|i| (delta, i, before.clone(), after.clone())),
            );
        }

        r.sort_by_key(|(delta, _, _, _)| *delta);

        r
    }

    fn search(&mut self) -> Option<usize> {
        // let mut lowest_visited = self.inner.clone();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: BinaryHeap<Reverse<Guess>> = BinaryHeap::from([Reverse(Guess {
            inner: self.inner.clone(),
            steps: 0,
        })]);
        while !queue.is_empty() {
            let Some(Reverse(guess)) = queue.pop() else {
                break;
            };
            if guess.inner == self.target {
                return Some(guess.steps);
            }
            // if lowest_visited.len() > guess.inner.len() {
            //     lowest_visited = guess.inner.clone();
            //     println!(
            //         "Queue: {}\nVisited: {}\nSteps: {}\nBest size: {}\nBest: {}\n\n",
            //         queue.len(),
            //         visited.len(),
            //         guess.steps,
            //         lowest_visited.len(),
            //         lowest_visited
            //     );
            // }
            self.inner = guess.inner;
            let replacements = self.replacements();
            for (_, i, before, after) in replacements.into_iter() {
                let mut new_guess = Guess {
                    inner: self.inner.clone(),
                    steps: guess.steps + 1,
                };
                replace(&mut new_guess.inner, i, &before, &after);
                if visited.contains(&new_guess.inner) {
                    continue;
                }
                visited.insert(new_guess.inner.clone());
                queue.push(Reverse(new_guess));
            }
        }

        None
    }
}

fn matches(haystack: &str, needle: &str) -> Vec<usize> {
    haystack
        .as_bytes()
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle.as_bytes())
        .map(|(i, _)| i)
        .collect()
}

fn replace(s: &mut String, i: usize, before: &str, after: &str) {
    let mut new = s[0..i].to_owned();
    new += after;
    new += &s[i + before.len()..s.len()];
    *s = new;
}

fn chars_reachable(start: &String, h: &HashMap<String, String>) -> HashSet<char> {
    print!("{} -> ", start);

    /*
     * 1. Start with e, branch out to as many chars as you can
     * 2. Start with *target*, branch out to as many chars as you can
     * 2. Take the intersection of #1 and #2
     * 3. Prune all kv pairs that contain a letter in the intersection
     *
     * abc -> d
     * dd -> e
     *
     * input: abzabz
     * abz -> f
     * z -> g
     * ff -> e
     *
     * e -> ff
     * f -> abz
     * {efabz}
     * for k, v in h {
     *  if !v.chars().all(* in reachable *) {
     *      *prune*
     * }
     * }
     *
     * for all keys, do any contain d. If yes, do
     */
    let mut reachable_chars = HashSet::new();
    reachable_chars.extend(start.chars());
    let mut new_reach = true;
    while new_reach {
        new_reach = false;
        for (before, after) in h.iter() {
            if before.chars().all(|c| reachable_chars.contains(&c)) {
                for c in after.chars() {
                    if !reachable_chars.contains(&c) {
                        new_reach = true;
                        reachable_chars.insert(c);
                    }
                }
            }
        }
    }

    reachable_chars
}

fn prune_graph(molecule: Molecule) -> HashMap<String, String> {
    let inner_reachable = chars_reachable(&molecule.inner, &molecule.rules.clone());
    let target_reachable = chars_reachable(
        &molecule.target,
        &molecule
            .rules
            .clone()
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect(),
    );

    let bad_chars: HashSet<char> = inner_reachable
        .symmetric_difference(&target_reachable)
        .map(|&c| c)
        .collect();

    molecule
        .rules
        .into_iter()
        .filter(|(before, after)| {
            before.chars().all(|c| !bad_chars.contains(&c))
                && after.chars().all(|c| !bad_chars.contains(&c))
        })
        .collect()
}

fn read_input(path: &str) -> Molecule {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let (rules_raw, input) = s.split_once("\n\n").expect("Input must have two parts");
    let rules = rules_raw
        .lines()
        .map(|line| {
            let (input, out) = line.trim().split_once(" => ").expect("Bad rule input");
            (out.to_string(), input.to_string())
        })
        .collect();
    Molecule {
        rules,
        target: "e".to_string(),
        inner: input.trim().to_string(),
    }
}

fn part_one(molecule: Molecule) -> usize {
    let mut molecule = molecule;
    molecule.rules = molecule
        .rules
        .clone()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();
    let mut visited = HashSet::new();
    for (_, i, b, a) in molecule.replacements() {
        let mut candidate = molecule.inner.clone();
        replace(&mut candidate, i, &b, &a);
        visited.insert(candidate);
    }
    visited.len()
}

fn part_two(molecule: Molecule) -> usize {
    let mut molecule = molecule;
    molecule.rules = prune_graph(molecule.clone());
    molecule.search().expect("Must be a path")
}

fn main() {
    let molecule = read_input("input.txt");
    println!("Part one: {}", part_one(molecule.clone()));
    println!("Part two: {}", part_two(molecule)); // Sometimes you get super lucky and A* finds the path like immediately (0.02 seconds)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prune() {
        let mut s = "abcdefg".to_owned();
        replace(&mut s, 2, "cde", "abc");
        assert_eq!(s, "ababcfg".to_owned());

        let h = HashMap::from([
            ("a".to_owned(), "b".to_owned()),
            ("abc".to_owned(), "b".to_owned()),
            ("b".to_owned(), "e".to_owned()),
            ("d".to_owned(), "f".to_owned()),
        ]);
        let p = prune_graph(Molecule {
            rules: h,
            inner: s,
            target: "e".to_string(),
        });
        assert!(!p.contains_key(&"d".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let molecule = read_input("test2.txt");
        println!("{:?}", molecule);
        assert_eq!(part_two(molecule), 6);
    }
}
