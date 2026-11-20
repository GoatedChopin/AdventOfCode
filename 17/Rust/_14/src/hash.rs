fn get_index(i: usize, list: &Vec<usize>) -> usize {
    i % list.len()
}

fn twist(list: &mut Vec<usize>, length: usize, current_position: usize) {
    for i in 0..length / 2 {
        let left = (current_position + i) % list.len();
        let right = (current_position + length - 1 - i) % list.len();

        list.swap(left, right);
    }
}

fn part_one(input: &[usize], ring_size: usize) -> usize {
    let mut list = (0..=ring_size).collect::<Vec<usize>>();
    let mut current = 0;
    let mut skip = 0;
    for length in input {
        twist(&mut list, *length, current);
        current = get_index(current + length + skip, &list);
        skip += 1;
    }
    list[0] * list[1]
}

fn compute_dense_hashes(list: &Vec<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for i in (0..list.len()).step_by(16) {
        let mut dense = list[i];
        for j in i + 1..i + 16 {
            dense ^= list[j];
        }
        out.push(dense);
    }
    out
}

pub fn hash(input: &Vec<u8>) -> String {
    let mut input = input.clone();
    input.extend([17, 31, 73, 47, 23].iter());
    let input: Vec<usize> = input.into_iter().map(|i| i.into()).collect();
    let mut list = (0..=255).collect::<Vec<usize>>();
    let mut current = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for length in input.iter() {
            twist(&mut list, *length, current);
            current = get_index(current + length + skip, &list);
            skip += 1;
        }
    }
    let dense_hashes = compute_dense_hashes(&list);

    dense_hashes.iter().map(|d| format!("{:02x}", d)).collect()
}
