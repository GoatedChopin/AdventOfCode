mod part_one;
mod part_two;
use part_one::process_input_one;
use part_two::process_input_two;
use std::fs;


fn main() {
    let input = fs::read_to_string("./inputs.txt").unwrap();
    print!("{}", input);
    // Part one
    print!("\nPart One:\n");
    print!("{}", process_input_one(&input));

    // Part two
    print!("\nPart Two:\n");
    print!("{}", process_input_two(&input));
}
