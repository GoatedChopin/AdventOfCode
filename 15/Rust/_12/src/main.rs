use std::collections::HashMap;
use std::fs;

enum Dict {
    HashMap(HashMap<String, Dict>),
    Vec(Vec<Dict>),
    String(String),
    Int(isize),
}

impl From<&str> for Dict {
    fn from(s: &str) -> Self {
        if s.len() == 0 {
            return Dict::HashMap(HashMap::new());
        }
        let mut dict = match s[0] {
            '{' => Dict::HashMap(HashMap::new()),
            '[' => Dict::Vec(Vec::new()),
            '"' => Dict::String(String::new()),
            '0' => Dict::Int(0),
            '1' => Dict::Int(0),
            '2' => Dict::Int(0),
            '3' => Dict::Int(0),
            '4' => Dict::Int(0),
            '5' => Dict::Int(0),
            '6' => Dict::Int(0),
            '7' => Dict::Int(0),
            '8' => Dict::Int(0),
            '9' => Dict::Int(0),
            _ => Dict::String(String::new()),
        };
        dict

        /*
         * 1. Find all open and close brackets so we can recursively parse
         * 2. Make a parsing function for each member of Dict
         * 3. Write test cases
         */
    }
}

fn read_input(path: &str) -> Dict {
    let string = fs::read_to_string(path)
        .expect("Input file must exist")
        .trim();
    Dict::from(string)
}

fn main() {
    println!("Hello, world!");
}
