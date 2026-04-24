use std::{collections::HashSet, fmt::Write, fs};

#[derive(Clone)]
struct Polymer<T> {
    head: Option<usize>,
    tail: Option<usize>,
    chain: Vec<T>,
    deleted: Vec<bool>,
    next_indices: Vec<Option<usize>>,
    prev_indices: Vec<Option<usize>>,
}

#[derive(Debug)]
enum PolymerError {
    IndexOutOfBounds,
}

impl Polymer<char> {
    fn from_str(s: &str) -> Self {
        Self::from_iter(s.chars())
    }
}

impl<T> Polymer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let chain: Vec<T> = iter.into_iter().collect();
        let len = chain.len();
        let deleted = vec![false; len];
        let mut next_indices: Vec<Option<usize>> = (0..len).map(|i| Some(i + 1)).collect();
        let mut prev_indices: Vec<Option<usize>> = (0..len).map(|i| i.checked_sub(1)).collect();
        if len > 0 {
            next_indices[len - 1] = None;
            prev_indices[0] = None;
        }
        Self {
            head: Some(0),
            tail: Some(len.saturating_sub(1)),
            chain,
            deleted,
            next_indices,
            prev_indices,
        }
    }

    fn remove_index(&mut self, i: usize) -> Result<(), PolymerError> {
        if i > self.chain.len() {
            return Err(PolymerError::IndexOutOfBounds);
        }

        self.deleted[i] = true;

        if let Some(h) = self.head
            && h == i
        {
            self.head = self.next_indices[i];
        }

        if let Some(t) = self.tail
            && t == i
        {
            self.tail = self.prev_indices[i];
        }

        let next = self.next_indices[i];
        let prev = self.prev_indices[i];

        if let Some(n) = next {
            self.prev_indices[n] = prev;
        }

        if let Some(p) = prev {
            self.next_indices[p] = next;
        }

        return Ok(());
    }

    fn next_index(&self, o: Option<usize>) -> Option<usize> {
        match o {
            Some(i) => self.next_indices[i],
            None => None,
        }
    }
}

impl std::fmt::Display for Polymer<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut head = self.head;
        while let Some(h) = head {
            let r = f.write_char(self.chain[h]);
            if let Err(e) = r {
                println!("{}", e);
                return r;
            }
            head = self.next_index(head);
        }
        Ok(())
    }
}

fn read_input(file_path: &str) -> Polymer<char> {
    let s = fs::read_to_string(file_path).unwrap();
    Polymer::from_str(&s)
}

fn opposite_polarity(c1: char, c2: char) -> bool {
    let upper1 = c1.to_ascii_uppercase();
    let upper2 = c2.to_ascii_uppercase();
    return upper1 == upper2 && c1 != c2;
}

fn part_one(poly: Polymer<char>) -> usize {
    if poly.chain.len() < 2 {
        return poly.chain.len();
    }

    let mut poly = poly;

    let mut clean = false;
    while !clean {
        clean = true;
        let mut left = poly.head;
        let mut right = poly.next_indices[left.unwrap()];
        // println!("{}", poly);
        while let Some(l) = left
            && let Some(r) = right
        {
            if opposite_polarity(poly.chain[l], poly.chain[r]) {
                // println!(
                //     "Removing {} and {} at index {}",
                //     poly.chain[l], poly.chain[r], l
                // );
                let _ = poly.remove_index(l);
                let _ = poly.remove_index(r);
                left = right;
                clean = false;
            }
            left = poly.next_index(left);
            right = poly.next_index(left);
        }
    }

    poly.deleted.iter().fold(0, |acc, b| match b {
        false => acc + 1,
        true => acc,
    })
}

fn part_two(poly: Polymer<char>) -> usize {
    let mut unique_chars = HashSet::new();
    poly.chain.iter().for_each(|c| {
        unique_chars.insert(c.to_ascii_lowercase());
    });

    let mut lowest_polymer = usize::MAX;
    for c in unique_chars.iter() {
        let filtered_chars = poly
            .chain
            .iter()
            .filter(|p| **p != *c && **p != c.to_ascii_uppercase())
            .map(|p| *p);
        let new_polymer = Polymer::from_iter(filtered_chars);
        let new_low = part_one(new_polymer);
        lowest_polymer = std::cmp::min(lowest_polymer, new_low);
    }
    lowest_polymer
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 10);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("test.txt");
        assert_eq!(part_two(input), 4);
    }
}
