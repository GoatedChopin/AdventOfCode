use std::collections::VecDeque;
use std::fs;

fn main() {
    let mut topmap: Vec<Vec<u8>> = fs::read_to_string("inputs/12.txt")
        .unwrap()
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'E' => 69,
                    'S' => 83,
                    _ => c as u8 - 97,
                })
                .collect()
        })
        .collect();

    let (start_row, start_col) = (0, 0);
    let (end_row, end_col) = (0, 0);

    for row in 0..topmap.len() {
        for col in 0..topmap[0].len() {
            if topmap[row][col] == 83 {
                start_row = row;
                start_col = col;
                topmap[row][col] = 97;
            } else if topmap[row][col] == 69 {
                end_row = row;
                end_col = col;
                topmap[row][col] = 122;
            }
        }
    }

    fn valid_neighbors(topmap: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<(usize, usize)> {
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        dirs.iter()
            .map(|(rd, cd)| (row as i32 + rd, col as i32 + cd))
            .filter(|(new_row, new_col)| (new_row, new_col)
