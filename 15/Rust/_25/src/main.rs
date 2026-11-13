use std::collections::HashMap;

struct Cache {
    inner: HashMap<usize, (usize, usize)>,
    max_i: usize,
    max_row: usize,
    max_col: usize,
}

fn next_number(n: usize) -> usize {
    n * 252533 % 33554393
}

fn coordinate(i: usize) -> (usize, usize) {
    let mut d = 0;

    while (d + 1) * (d + 2) / 2 <= i {
        d += 1;
    }

    let offset = i - d * (d + 1) / 2;

    (d - offset, offset)
}

fn part_one(c: (usize, usize), cache: &mut Cache) -> usize {
    let mut i = 0;
    while coordinate(i) != c {
        i += 1;
    }
    let mut current = 20151125;
    for _ in 0..i {
        current = next_number(current);
    }
    current
}

fn main() {
    let coordinate = (2978 - 1, 3083 - 1);
    let mut cache = Cache {
        inner: HashMap::new(),
        max_i: 0,
        max_row: 0,
        max_col: 0,
    };
    println!("Part one: {}", part_one(coordinate, &mut cache));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coordinates() {
        let mut cache = Cache {
            inner: HashMap::new(),
            max_i: 0,
            max_row: 0,
            max_col: 0,
        };
        assert_eq!(coordinate(5), (1, 1));
        assert_eq!(coordinate(21), (0, 5));
    }
}
