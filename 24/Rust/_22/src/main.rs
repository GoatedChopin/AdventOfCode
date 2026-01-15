use std::{
    collections::{HashMap},
    fs,
};

fn mix(secret_number: isize, number: isize) -> isize {
    secret_number ^ number
}

fn prune(secret_number: isize) -> isize {
    secret_number % 16777216
}

#[derive(Debug, Clone)]
struct Buyer {
    secret_number: isize,
    last_secret_number: Option<isize>,
    prices: Vec<isize>,
    deltas: Vec<isize>,
    cache: HashMap<Vec<isize>, isize>,
}

impl Buyer {
    fn new(number: isize) -> Self {
        Self {
            secret_number: number,
            last_secret_number: None,
            prices: vec![number % 10], // Initial price before any changes
            deltas: Vec::new(),
            cache: HashMap::new(),
        }
    }

    fn price(&self, secret_number: isize) -> isize {
        secret_number % 10
    }

    fn next(&mut self) -> Self {
        self.last_secret_number = Some(self.secret_number);

        // multiply by 64
        let new_number = self.secret_number * 64;

        // mix
        self.secret_number = mix(self.secret_number, new_number);

        // prune
        self.secret_number = prune(self.secret_number);

        // divide by 32
        let new_number = self.secret_number / 32;

        // mix
        self.secret_number = mix(self.secret_number, new_number);

        // prune
        self.secret_number = prune(self.secret_number);

        // multiply by 2048
        let new_number = self.secret_number * 2048;

        // mix
        self.secret_number = mix(self.secret_number, new_number);

        // prune
        self.secret_number = prune(self.secret_number);

        let new_price = self.price(self.secret_number);
        self.deltas
            .push(new_price - self.price(self.last_secret_number.unwrap()));
        self.prices.push(new_price);

        Self {
            secret_number: self.secret_number,
            last_secret_number: self.last_secret_number,
            prices: self.prices.clone(),
            deltas: self.deltas.clone(),
            cache: self.cache.clone(),
        }
    }

    fn get_price(&mut self, pattern: &[isize]) -> isize {
        if self.cache.contains_key(pattern) {
            return *self.cache.get(pattern).unwrap();
        }
        for i in 0..=(self.deltas.len() - pattern.len()) {
            let slice = self.deltas[i..i + pattern.len()].to_vec();
            let price = self.prices[i + pattern.len()];
            if !self.cache.contains_key(&slice) {
                self.cache.insert(slice.clone(), price);
            }
            if slice == pattern {
                return price;
            }
        }
        self.cache.insert(pattern.to_vec(), 0);
        0
    }
}

fn read_input(file_path: &str) -> Vec<Buyer> {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
        .lines()
        .map(|line| Buyer::new(line.parse().unwrap()))
        .collect()
}

fn part_one(input: Vec<Buyer>) -> isize {
    let mut buyers = input.clone();
    for buyer in buyers.iter_mut() {
        for _ in 0..2000 {
            *buyer = buyer.next();
        }
    }
    buyers.iter().map(|buyer| buyer.secret_number).sum()
}

fn part_two(input: Vec<Buyer>, slice_size: usize) -> isize {
    let mut buyers = input.clone();
    let mut slice_totals = HashMap::new();
    for i in 0..2000 {
        for buyer in buyers.iter_mut() {
            *buyer = buyer.next();
            if i > slice_size {
                let slice = buyer.deltas[i-slice_size..i].to_vec();
                if buyer.cache.contains_key(&slice) {
                    continue;
                }
                slice_totals.insert(
                    slice.clone(),
                    slice_totals.get(&slice).unwrap_or(&0) + buyer.prices[i],
                );
                buyer.cache.insert(slice, buyer.prices[i]);
            }
        }
    }
    *slice_totals.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(0, 0), 0);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 37327623);
    }

    #[test]
    fn test_part_two_individuals() {
        let mut buyer = Buyer::new(123);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[0], -3);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[1], 6);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[2], -1);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[3], -1);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[4], 0);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[5], 2);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[6], -2);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[7], 0);
        buyer = buyer.next();
        assert_eq!(buyer.deltas[8], -2);

        // Pattern [-1, -1, 0, 2] should give price 6 (the "second 6" per the problem)
        assert_eq!(buyer.get_price(&[-1, -1, 0, 2]), 6);

        let mut buyers = vec![
            Buyer::new(1),
            Buyer::new(2),
            Buyer::new(3),
            Buyer::new(2024),
        ];
        for _ in 0..2000 {
            buyers = buyers.iter_mut().map(|buyer| buyer.next()).collect();
        }
        assert_eq!(buyers[0].deltas.len(), 2000);
        assert_eq!(buyers[0].get_price(&[-2, 1, -1, 3]), 7);
        assert_eq!(buyers[1].get_price(&[-2, 1, -1, 3]), 7);
        assert_eq!(buyers[2].get_price(&[-2, 1, -1, 3]), 0);
        assert_eq!(buyers[3].get_price(&[-2, 1, -1, 3]), 9);
    }

    #[test]
    fn test_part_two() {
        // Part 2 has different test inputs: 1, 2, 3, 2024 (not 1, 10, 100, 2024)
        let input = vec![
            Buyer::new(1),
            Buyer::new(2),
            Buyer::new(3),
            Buyer::new(2024),
        ];
        assert_eq!(part_two(input, 4), 23);
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input, 4));
}
