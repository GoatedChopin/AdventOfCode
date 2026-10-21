use std::collections::HashMap;
use std::fs;

enum Dict {
    HashMap(HashMap<String, Dict>),
    Vec(Vec<Dict>),
    String(String),
    Int(isize),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum BracketMode {
    Open,
    Close,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Bracket {
    Curly(BracketMode),
    Square(BracketMode),
}

impl Bracket {
    fn matches(&self, right: &Bracket) -> bool {
        use Bracket::*;
        use BracketMode::*;
        match self {
            Curly(Open) => *right == Curly(Close),
            Square(Open) => *right == Square(Close),
            _ => false, // we only count a match if it's left open, right closed
        }
    }
    fn matches_char(&self, c: char) -> bool {
        match self {
            Bracket::Curly(BracketMode::Open) => c == '}',
            Bracket::Square(BracketMode::Open) => c == ']',
            Bracket::Curly(BracketMode::Close) => c == '{',
            Bracket::Square(BracketMode::Close) => c == '[',
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Bracket, Self::Error> {
        match c {
            '{' => Ok(Bracket::Curly(BracketMode::Open)),
            '[' => Ok(Bracket::Square(BracketMode::Open)),
            '}' => Ok(Bracket::Curly(BracketMode::Close)),
            ']' => Ok(Bracket::Square(BracketMode::Close)),
            _ => Err("Bad input char"),
        }
    }
}

impl From<&str> for Dict {
    fn from(s: &str) -> Self {
        if s.len() == 0 {
            return Dict::HashMap(HashMap::new());
        }
        let chars: Vec<char> = s.chars().collect();
        let mut dict = match chars[0] {
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
        let brackets: Vec<(usize, Bracket)> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| {
                let Ok(bracket_result) = Bracket::try_from(c) else {
                    return None;
                };
                Some((i, bracket_result))
            })
            .collect();
        println!("{:?}", brackets);

        if brackets.len() == 0 {
            // Immediately parse the whole primitive and return.
        }

        if brackets.len() == 2 {
            // Immediately parse the whole data structure
        }

        let mut i = 0;
        while i < brackets.len() - 1 {
            let j = i + 1;
            let (left_index, left_bracket) = brackets[i];
            let (right_index, right_bracket) = brackets[j];
            if !left_bracket.matches(&right_bracket) {
                continue;
            }
            i += 1;
        }
        dict

        /*
         * 1. Find all open and close brackets so we can recursively parse
         * 2. Make a parsing function for each member of Dict
         * 3. Write test cases
         */
    }
}

fn read_input(path: &str) -> Dict {
    let string = fs::read_to_string(path).expect("Input file must exist");
    Dict::from(string.trim())
}

fn main() {
    let dict = read_input("input.txt");
}
