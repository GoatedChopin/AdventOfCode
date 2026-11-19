use std::fs;

#[derive(Clone, Copy)]
struct Generator {
    name: char,
    state: usize,
    factor: usize,
    remainder: usize,
    divisor: usize,
}

impl Generator {
    fn new(name: char, state: usize, factor: usize, remainder: usize, divisor: usize) -> Self {
        Self {
            name,
            state,
            factor,
            remainder,
            divisor,
        }
    }

    fn step(&mut self) -> usize {
        self.state = self.state * self.factor % self.remainder;
        self.state
    }

    fn step_to_divisible(&mut self) -> usize {
        self.step();
        while self.state % self.divisor != 0 {
            self.state = self.state * self.factor % self.remainder;
        }
        self.state
    }
}

fn read_input(path: &str) -> Vec<Generator> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.lines()
        .zip([16807, 48271])
        .zip([4, 8])
        .map(|((line, factor), divisor)| {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            Generator::new(
                parts[1].parse().expect("Bad name"),
                parts[4].parse().expect("Bad initial state"),
                factor,
                2147483647,
                divisor,
            )
        })
        .collect()
}

fn bits_match(a: usize, b: usize) -> bool {
    let ab = format!("{:32b}", a);
    let bb = format!("{:32b}", b);

    if ab[ab.len() - 16..ab.len()] != bb[bb.len() - 16..bb.len()] {
        return false;
    }

    return true;
}

fn part_one(generators: &[Generator]) -> usize {
    let mut matches = 0;
    let mut generators: Vec<Generator> = generators.clone().iter().map(|g| *g).collect();
    for _ in 0..40_000_000 {
        generators.iter_mut().for_each(|generator| {
            generator.step();
        });
        if bits_match(generators[0].state, generators[1].state) {
            matches += 1;
        }
    }
    matches
}

fn part_two(generators: &[Generator]) -> usize {
    let mut matches = 0;
    let mut generators: Vec<Generator> = generators.iter().map(|g| *g).collect();
    for _ in 0..5_000_000 {
        generators.iter_mut().for_each(|generator| {
            generator.step_to_divisible();
        });
        if bits_match(generators[0].state, generators[1].state) {
            matches += 1;
        }
    }
    matches
}

fn main() {
    let generators = read_input("input.txt");
    println!("Part one: {}", part_one(&generators));
    println!("Part two: {}", part_two(&generators));
}
