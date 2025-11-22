use std::fs;
use std::collections::HashMap;

fn read_input(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_string()
}

fn find_spiral_number(number: usize) -> usize {
    let mut steps = 1;
    let mut iterations = 0;
    let mut x = 0;
    let mut y = 0;
    let mut radius = 1;
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_index = 0;
    while steps < number {
        for _ in 0..radius {
            steps += 1;
            x += directions[direction_index].0;
            y += directions[direction_index].1;
            if steps == number {
                break;
            }
        }
        // println!("steps: {}, x: {}, y: {}, radius: {}, direction_index: {}", steps, x, y, radius, direction_index);
        direction_index = (direction_index + 1) % directions.len();
        iterations += 1;
        if iterations % 2 == 0 {
            radius += 1;
        }
    }
    let result = (x.abs() + y.abs()) as usize;
    // println!("x: {}, y: {}, result: {}", x, y, result);
    result
}

fn find_additive_spiral_number(number: usize) -> usize {
    let mut steps = 1;
    let mut iterations = 0;
    let mut x = 0;
    let mut y = 0;
    let mut radius = 1;
    let mut grid: HashMap<(i32, i32), usize> = HashMap::new();
    grid.insert((0, 0), 1);
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut direction_index = 0;
    while steps < number {
        for _ in 0..radius {
          let mut sum = 0;
          for nx in x-1..x+2 {
            for ny in y-1..y+2 {
              sum += grid.get(&(nx, ny)).unwrap_or(&0);
            }
          }
          grid.insert((x, y), sum);
          if sum > number {
            return sum;
          }
          steps += 1;
          x += directions[direction_index].0;
          y += directions[direction_index].1;
        }
        direction_index = (direction_index + 1) % directions.len();
        iterations += 1;
        if iterations % 2 == 0 {
            radius += 1;
        }
    }
    return 0;
}

fn main() {
    test();
    let input = read_input("input.txt");
    let number = input.parse::<usize>().unwrap();
    println!("Part one: {}", find_spiral_number(number));
    println!("Part two: {}", find_additive_spiral_number(number));
}

fn test() {
    assert!(find_spiral_number(1) == 0);
    assert!(find_spiral_number(12) == 3);
    assert!(find_spiral_number(23) == 2);
    assert!(find_spiral_number(1024) == 31);
}