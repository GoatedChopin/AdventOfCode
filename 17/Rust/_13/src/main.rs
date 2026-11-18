use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Firewall {
    packet: isize,
    scanners: HashMap<isize, Scanner>,
}

impl Firewall {
    fn delay(&mut self) {
        for (_, value) in self.scanners.iter_mut() {
            match value.guard_direction {
                Direction::Down => {
                    value.guard_position += 1;
                    if value.guard_position == value.size - 1 {
                        value.guard_direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    value.guard_position = value.guard_position.saturating_sub(1);
                    if value.guard_position == 0 {
                        value.guard_direction = Direction::Down;
                    }
                }
            }
        }
    }

    fn step(&mut self) -> Result<isize, ()> {
        self.packet += 1;
        let mut error = false;
        if let Some(scanner) = self.scanners.get(&self.packet) {
            if scanner.guard_position == 0 {
                error = true;
            }
        }

        for (_, value) in self.scanners.iter_mut() {
            match value.guard_direction {
                Direction::Down => {
                    value.guard_position += 1;
                    if value.guard_position == value.size - 1 {
                        value.guard_direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    value.guard_position = value.guard_position.saturating_sub(1);
                    if value.guard_position == 0 {
                        value.guard_direction = Direction::Down;
                    }
                }
            }
        }
        if error {
            return Err(());
        }
        return Ok(self.packet);
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone)]
struct Scanner {
    size: isize,
    guard_position: isize,
    guard_direction: Direction,
}

fn read_input(path: &str) -> Firewall {
    Firewall {
        packet: -1,
        scanners: fs::read_to_string(path)
            .expect("Input file must exist")
            .lines()
            .map(|line| {
                let Some((position, size)) = line.trim().split_once(": ") else {
                    panic!("Bad input line")
                };
                (
                    position.parse().expect("Bad position"),
                    Scanner {
                        size: size.parse().expect("Bad size"),
                        guard_position: 0,
                        guard_direction: Direction::Down,
                    },
                )
            })
            .collect(),
    }
}

fn part_one(firewall: Firewall) -> isize {
    let mut firewall = firewall;
    let mut total = 0;
    for _ in 0..10000 {
        match firewall.step() {
            Err(()) => {
                let Some(scanner) = firewall.scanners.get(&firewall.packet) else {
                    continue;
                };
                println!(
                    "{} * {} = {}",
                    firewall.packet,
                    scanner.size,
                    firewall.packet * scanner.size
                );
                total += firewall.packet * scanner.size;
            }
            Ok(_) => {}
        }
    }
    total
}

fn part_two(firewall: Firewall) -> isize {
    let Some((max_scanner, _)) = firewall
        .scanners
        .iter()
        .max_by(|(ai, _), (bi, _)| ai.cmp(bi))
    else {
        return 0;
    };
    let mut delay = -1;
    loop {
        let caught = firewall.scanners.iter().any(|(index, scanner)| {
            let period = 2 * (scanner.size - 1);
            (delay + index) % period == 0
        });
        if !caught {
            break;
        }
        delay += 1
    }
    delay
}

fn main() {
    let firewall = read_input("input.txt");
    println!("Part one: {}", part_one(firewall.clone()));
    println!("Part two: {}", part_two(firewall));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let firewall = read_input("test.txt");
        assert_eq!(part_one(firewall), 24);
    }
}
