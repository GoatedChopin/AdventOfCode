use std::collections::BinaryHeap;

pub fn process_input_one(input: &str) -> u32 {
    let mut heap = BinaryHeap::<u32>::new();
    let mut quant = 0;
    let mut current;
    for chunk in input.split("\n\n") {
        for i in chunk.lines() {
            current = i.parse::<u32>().unwrap();
            quant += current;
        }
        heap.push(quant);
        quant = 0;
    }
    quant = heap.pop().unwrap();
    quant
}