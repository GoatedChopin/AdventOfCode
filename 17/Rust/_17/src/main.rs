use std::collections::VecDeque;

#[derive(Clone)]
struct Spinlock {
    buffer: VecDeque<usize>,
    cursor: usize,
    step: usize,
    max: usize,
}

impl Spinlock {
    fn new(step: usize) -> Self {
        Self {
            buffer: VecDeque::from([0]),
            cursor: 0,
            max: 0,
            step,
        }
    }

    fn next_cursor(&self) -> usize {
        (self.cursor + self.step) % self.buffer.len() + 1
    }

    fn step(&mut self) {
        self.max += 1;
        self.cursor = self.next_cursor();
        self.buffer.insert(self.cursor, self.max);
    }
}

fn part_one(spinlock: Spinlock) -> usize {
    let mut spinlock = spinlock;
    for _ in 0..=2017 {
        spinlock.step();
    }
    let buffer_size = spinlock.buffer.len();
    for (i, u) in spinlock.buffer.iter().enumerate() {
        if *u != 2017 {
            continue;
        }
        return match i == buffer_size - 1 {
            true => spinlock.buffer[0],
            false => spinlock.buffer[i + 1],
        };
    }
    return 0;
}

fn part_two(spinlock: Spinlock, steps: usize) -> usize {
    let mut cursor = 0;
    let mut buffer_size = 1;
    let mut next_to_zero = 0;
    for i in 1..=steps {
        cursor = (cursor + spinlock.step) % buffer_size + 1;
        if cursor == 1 {
            next_to_zero = i;
        }
        buffer_size += 1;
    }
    next_to_zero
}

fn main() {
    let input = 386;
    let spinlock = Spinlock::new(input);

    println!("Part one: {}", part_one(spinlock.clone()));
    println!("Part two: {}", part_two(spinlock, 50_000_000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spinlock_step() {
        let mut spinlock = Spinlock::new(3);
        spinlock.step();
        assert_eq!(spinlock.buffer, [0, 1]);
        spinlock.step();
        assert_eq!(spinlock.buffer, [0, 2, 1]);
        spinlock.step();
        assert_eq!(spinlock.buffer, [0, 2, 3, 1]);
        spinlock.step();
        assert_eq!(spinlock.buffer, [0, 2, 4, 3, 1]);
    }

    #[test]
    fn test_part_two() {
        let spinlock = Spinlock::new(3);
        assert_eq!(part_two(spinlock, 9), 0);
    }
}
