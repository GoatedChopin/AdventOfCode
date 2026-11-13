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

fn coordinate(i: usize, cache: &mut Cache) -> (usize, usize) {
    if let Some(c) = cache.inner.get(&i) {
        return *c;
    }
    let (mut row, mut col) = match cache.inner.get(&cache.max_i) {
        Some(&(row, col)) => (row, col),
        None => (0, 0),
    };
    for _ in cache.max_i..i {
        if row == 0 {
            col = 0;
            row = cache.max_row + 1;
            cache.max_row += 1;
        } else {
            row -= 1;
            col += 1;
        }
    }
    cache.inner.insert(i, (row, col));
    cache.max_i = cache.max_i.max(i);
    (row, col)
}

fn part_one(c: (usize, usize), cache: &mut Cache) -> usize {
    let mut i = 0;
    while coordinate(i, cache) != c {
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
        assert_eq!(coordinate(5, &mut cache), (1, 1));
        assert_eq!(coordinate(21, &mut cache), (0, 5));
    }
}
