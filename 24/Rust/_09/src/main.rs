use std::collections::VecDeque;
use std::fs;

#[derive(Clone)]
struct Block {
    position: usize,
    size: usize,
}

#[derive(Clone)]
struct FileSystem {
    memory: VecDeque<Option<usize>>,
    empty_blocks: VecDeque<Block>,
    file_blocks: VecDeque<Block>,
}

fn read_input(file_path: &str) -> FileSystem {
    let file_content = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut memory = VecDeque::new();
    let mut empty_blocks = VecDeque::new();
    let mut file_blocks = VecDeque::new();
    let mut file_id = 0;
    let mut is_file = true;
    for char in file_content.trim().chars() {
        let num = char.to_digit(10).unwrap();
        if is_file {
            file_blocks.push_back(Block {
                position: memory.len(),
                size: num as usize,
            });
            for _ in 0..num {
                memory.push_back(Some(file_id));
            }
            file_id += 1;
        } else {
            empty_blocks.push_back(Block {
                position: memory.len(),
                size: num as usize,
            });
            for _ in 0..num {
                memory.push_back(None)
            }
        }
        is_file = !is_file;
    }
    FileSystem {
        memory,
        empty_blocks,
        file_blocks,
    }
}

fn optimize_memory(file_system: &mut FileSystem) {
    for file_block in file_system.file_blocks.iter_mut().rev() {
        for empty_block in file_system.empty_blocks.iter_mut() {
            if empty_block.position < file_block.position && empty_block.size >= file_block.size {
                // Swap the actual blocks in memory
                for i in 0..file_block.size {
                    file_system
                        .memory
                        .swap(file_block.position + i, empty_block.position + i);
                }
                file_block.position = empty_block.position;
                empty_block.position = empty_block.position + file_block.size;
                empty_block.size = empty_block.size - file_block.size;
                break;
            }
        }
    }
}

fn hash(file_system: &FileSystem) -> usize {
    let mut hash = 0;
    for (i, data) in file_system.memory.iter().enumerate() {
        if let Some(num) = data {
            // print!("{}", num);
            hash += i * num;
        } else {
            // print!(".");
        }
    }
    // println!();
    hash
}

fn part_two(file_system: &FileSystem) -> usize {
    let mut tmp_file_system = file_system.clone();
    optimize_memory(&mut tmp_file_system);
    hash(&tmp_file_system)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_two() {
        let file_system = read_input("test.txt");
        assert_eq!(part_two(&file_system), 2858);
    }
}

fn main() {
    let file_system = read_input("input.txt");
    println!("Part two: {}", part_two(&file_system));
}
