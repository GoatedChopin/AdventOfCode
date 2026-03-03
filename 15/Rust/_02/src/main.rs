use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoxDims(u32, u32, u32);

impl BoxDims {
    fn from_str(s: &str) -> Option<Self> {
        let mut parts = s.trim().split('x');
        let l = parts.next()?.parse().ok()?;
        let w = parts.next()?.parse().ok()?;
        let h = parts.next()?.parse().ok()?;
        Some(BoxDims(l, w, h))
    }

    fn areas(&self) -> (u32, u32, u32) {
        let BoxDims(l, w, h) = *self;
        (l*w, w*h, h*l)
    }

    fn perimeters(&self) -> (u32, u32, u32) {
        let BoxDims(l, w, h) = *self;
        (2*(l+w), 2*(w+h), 2*(h+l))
    }

    fn volume(&self) -> u32 { self.0 * self.1 * self.2 }
}

fn parse_input(input: &str) -> Vec<BoxDims> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| BoxDims::from_str(l).expect("invalid line"))
        .collect()
}

fn part_one(boxes: &[BoxDims]) -> u32 {
    boxes.iter().map(|b| {
        let (a1,a2,a3) = b.areas();
        let surface = 2*a1 + 2*a2 + 2*a3;
        let slack = a1.min(a2).min(a3);
        surface + slack
    }).sum()
}

fn part_two(boxes: &[BoxDims]) -> u32 {
    boxes.iter().map(|b| {
        let (p1,p2,p3) = b.perimeters();
        let ribbon = p1.min(p2).min(p3);
        ribbon + b.volume()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let b1 = parse_input("2x3x4");
        assert_eq!(part_one(&b1), 58);
        let b2 = parse_input("1x1x10");
        assert_eq!(part_one(&b2), 43);
    }

    #[test]
    fn examples_part2() {
        let b1 = parse_input("2x3x4");
        assert_eq!(part_two(&b1), 34);
        let b2 = parse_input("1x1x10");
        assert_eq!(part_two(&b2), 14);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input.txt");
    let boxes = parse_input(&input);
    println!("Part one: {}", part_one(&boxes));
    println!("Part two: {}", part_two(&boxes));
}
