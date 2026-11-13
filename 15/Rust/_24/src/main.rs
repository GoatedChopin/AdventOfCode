use std::{cmp::Ordering, fs};

fn distribution(packages: Vec<usize>, groups: Vec<usize>) -> Vec<usize> {
    let mut out = Vec::from([0, 0, 0]);
    packages
        .iter()
        .zip(groups.iter())
        .for_each(|(v, g)| out[*g] += *v);
    out
}

fn quantum_entanglement(group: usize, packages: Vec<usize>, groups: Vec<usize>) -> usize {
    packages
        .iter()
        .zip(groups.iter())
        .fold(0, |acc, (v, g)| if *g == group { acc * *v } else { acc })
}

fn create_balanced_group(packages: Vec<usize>) -> Vec<usize> {
    let mut backtrack_point = packages.len() - 1;
    let mut untenable: Vec<bool> = packages.clone().iter().map(|_| false).collect();
    let mut in_group: Vec<bool> = packages.clone().iter().map(|_| false).collect();

    // We know that if we pack the biggest packages into the first group and find a solution,
    // we've found the optimal solution. So, if we start by greedily picking the biggest packages
    // and try to balance the groups, we'll either find the right solution or backtrack
    //
    // ... we also probably don't have to sort into three groups.
    // We can sort the items into "in" vs "not in" and then recurse with the remaining numbers

    let mut pointer = backtrack_point;
    let mut total = 0;
    let target: usize = packages.iter().map(|i| *i).sum::<usize>() / 3;

    while total != target {
        match total.cmp(&target) {
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                // Add the next number
                total += packages[pointer];
                in_group[pointer] = true;
                pointer = pointer.saturating_sub(1);
            }
            Ordering::Greater => {
                // backtrack
                backtrack_point = pointer;
                while backtrack_point < untenable.len() && untenable[backtrack_point] {
                    backtrack_point += 1;
                }
                if backtrack_point < untenable.len() {
                    untenable[backtrack_point] = true;
                    in_group[backtrack_point] = false;
                    total -= packages[backtrack_point];
                }
                for i in 0..backtrack_point {
                    untenable[i] = false;
                }
                pointer = backtrack_point.saturating_sub(1);
            }
        }
    }

    packages
        .iter()
        .zip(in_group.iter())
        .filter_map(|(p, ig)| if *ig { Some(*p) } else { None })
        .collect()
}

fn read_input(path: &str) -> Vec<usize> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let mut out: Vec<usize> = s
        .lines()
        .map(|line| line.trim().parse().expect("Bad weight"))
        .collect();
    out.sort();
    out
}

fn part_one(packages: Vec<usize>) -> usize {
    let group = create_balanced_group(packages);
    println!("{:?}", group);
    group.into_iter().fold(1, |acc, i| acc * i)
}

fn main() {
    let packages = read_input("input.txt");
    println!("Part one: {}", part_one(packages));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut packages: Vec<usize> = Vec::new();
        for i in 1..=5 {
            packages.push(i);
        }
        for i in 7..=11 {
            packages.push(i);
        }
        assert_eq!(part_one(packages), 99);
    }
}
