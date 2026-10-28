use std::collections::HashMap;
use std::fs;

fn read_input(path: &str) -> Vec<Vec<bool>> {
    let s = fs::read_to_string(path).expect("File must exist");
    s.lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn num_neighbors(p: (usize, usize), lights: &Vec<Vec<bool>>) -> usize {
    let mut n = 0;
    let rows = lights.len();
    let cols = lights[0].len();
    let top = p.0.saturating_sub(1);
    let left = p.1.saturating_sub(1);
    let bottom = (p.0 + 1).min(rows - 1);
    let right = (p.1 + 1).min(cols - 1);
    for r in top..=bottom {
        for c in left..=right {
            if (r, c) == p {
                // println!("Skipping self");
                continue;
            }
            if let Some(l) = lights.get(r).unwrap_or(&Vec::new()).get(c) {
                if !l {
                    continue;
                }
                n += 1;
                continue;
            }
            // println!("Bad coordinate")
        }
    }
    n
}

fn display(lights: &Vec<Vec<bool>>) {
    lights.iter().for_each(|row| {
        row.iter()
            .for_each(|light| if *light { print!("#") } else { print!(".") });
        println!();
    });
}

fn display_neighbors(lights: &Vec<Vec<bool>>) {
    lights.iter().enumerate().for_each(|(r, row)| {
        row.iter()
            .enumerate()
            .for_each(|(c, _)| print!("{}", num_neighbors((r, c), lights)));
        println!();
    });
}

fn step(lights: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_lights = lights.clone();
    for (r, row) in lights.iter().enumerate() {
        for (c, light) in row.iter().enumerate() {
            let n = num_neighbors((r, c), &lights);
            new_lights[r][c] = match (light, n) {
                (true, 2) => true,
                (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    new_lights
}

fn part_one(steps: usize, lights: Vec<Vec<bool>>) -> usize {
    let mut lights = lights;
    for _ in 0..steps {
        lights = step(lights);
    }
    lights
        .iter()
        .flat_map(|v| v.iter().map(|b| if *b { 1 } else { 0 }))
        .sum()
}

fn corners(lights: &Vec<Vec<bool>>) -> HashMap<(usize, usize), bool> {
    let mut map = HashMap::new();
    let rows = lights.len();
    let cols = lights.get(0).unwrap_or(&Vec::new()).len();
    map.insert((0, 0), true);
    map.insert((0, cols - 1), true);
    map.insert((rows - 1, 0), true);
    map.insert((rows - 1, cols - 1), true);
    map
}

fn step_clamped(lights: Vec<Vec<bool>>, clamps: &HashMap<(usize, usize), bool>) -> Vec<Vec<bool>> {
    let mut new_lights = lights.clone();
    for (r, row) in lights.iter().enumerate() {
        for (c, light) in row.iter().enumerate() {
            if let Some(val) = clamps.get(&(r, c)) {
                new_lights[r][c] = *val;
                continue;
            }
            let n = num_neighbors((r, c), &lights);
            new_lights[r][c] = match (light, n) {
                (true, 2) => true,
                (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    new_lights
}

fn part_two(steps: usize, lights: Vec<Vec<bool>>) -> usize {
    let mut lights = lights;
    let c = corners(&lights);
    for (p, val) in c.iter() {
        lights[p.0][p.1] = *val;
    }
    for _ in 0..steps {
        lights = step_clamped(lights, &c);
    }
    lights
        .iter()
        .flat_map(|v| v.iter().map(|b| if *b { 1 } else { 0 }))
        .sum()
}

fn main() {
    let lights = read_input("input.txt");
    println!("Part one: {}", part_one(100, lights.clone()));
    println!("Part two: {}", part_two(100, lights));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let lights = read_input("test.txt");
        assert_eq!(part_one(4, lights), 4);
    }

    #[test]
    fn test_part_two() {
        let lights = read_input("test.txt");
        assert_eq!(part_two(5, lights), 17);
    }
}
