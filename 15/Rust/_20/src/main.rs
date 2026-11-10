fn part_one(target: usize) -> usize {
    let max_house = target / 10;

    let mut houses = vec![0; max_house + 1];

    for elf in 1..=max_house {
        for house in (elf..=max_house).step_by(elf) {
            houses[house] += elf * 10;
        }
    }

    houses
        .iter()
        .position(|&presents| presents >= target)
        .unwrap()
}

fn part_two(target: usize) -> usize {
    let max_house = target / 10;

    let mut houses = vec![0; max_house + 1];

    for elf in 1..=max_house {
        for house in (elf..=max_house).step_by(elf).take(50) {
            houses[house] += elf * 11;
        }
    }

    houses
        .iter()
        .position(|&presents| presents >= target)
        .unwrap()
}

fn main() {
    let target = 34000000;
    println!("Part one: {}", part_one(target));
    println!("Part two: {}", part_two(target));
}
