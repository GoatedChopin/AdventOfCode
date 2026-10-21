use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
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

impl TryFrom<&str> for Dict {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Dict, Self::Error> {
        Dict::try_from(s.to_string())
    }
}

impl TryFrom<String> for Dict {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Dict, Self::Error> {
        if s.len() == 0 {
            return Err("Empty string");
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
        let mut top_level_keys = HashMap::new();
        let mut depth = 0;
        let mut max_depth = 0;
        let mut open_quote = None;
        let mut close_quote = None;
        let mut open_region = None;
        for (i, &c) in chars.iter().enumerate() {
            let bracket = Bracket::try_from(c);
            match bracket {
                Ok(Bracket::Curly(BracketMode::Open)) => {
                    if depth == 1 {
                        open_region = Some(i);
                    }
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                Ok(Bracket::Curly(BracketMode::Close)) => {
                    depth -= 1;
                    if depth == 1 {
                        let open = open_quote.expect("No top level key close");
                        let close = close_quote.expect("No top level key close");
                        let key: String = chars[open..=close].into_iter().collect();
                        let substring: String = chars[open_region.expect("No region open")..=i]
                            .iter()
                            .collect();
                        let Ok(value) = Dict::try_from(substring) else {
                            return Err("Bad sub region");
                        };
                        println!("Keypair {:?} {:?}", key, value);
                        top_level_keys.insert(key.replace('"', ""), value);
                        open_quote = None;
                        close_quote = None;
                    }
                }
                Ok(Bracket::Square(BracketMode::Open)) => {
                    if depth == 1 {
                        open_region = Some(i + 1);
                    }
                    depth += 1;
                }
                Ok(Bracket::Square(BracketMode::Close)) => {
                    depth -= 1;
                    if depth == 1 {
                        let is_value = open_quote.is_some() && close_quote.is_some();
                        let substring: String = chars[open_region.expect("No region open")..i]
                            .iter()
                            .collect();
                        if substring.contains("[") {
                            let res = Dict::try_from(substring);
                            return match res {
                                Ok(d) => Ok(Dict::Vec(Vec::from([d]))),
                                Err(e) => Err(e),
                            };
                        }
                        let items: Vec<Dict> = substring
                            .split(",")
                            .into_iter()
                            .filter_map(|item| {
                                let Ok(d) = Dict::try_from(item) else {
                                    return None;
                                };
                                Some(d)
                            })
                            .collect();
                        if !is_value {
                            return Ok(Dict::Vec(items));
                        }
                        let open = open_quote.expect("No top level key close");
                        let close = close_quote.expect("No top level key close");
                        let key: String = chars[open..=close].into_iter().collect();
                        top_level_keys.insert(key, Dict::Vec(items));
                    }
                }
                _ => {}
            }
            if depth > 1 {
                continue;
            }
            if c == '"' {
                match (open_quote, close_quote) {
                    (None, None) => open_quote = Some(i + 1),
                    (Some(_), None) => close_quote = Some(i - 1),
                    _ => {}
                }
            }
        }

        if brackets.len() == 0 {
            // Immediately parse the whole primitive and return.
            match dict {
                Dict::Int(_) => {
                    let Ok(i) = s.parse() else {
                        return Err("Bad integer value");
                    };
                    return Ok(Dict::Int(i));
                }
                Dict::String(_) => return Ok(Dict::String(s.replace('"', "").to_string())),
                _ => return Err("Lack of Brackets"),
            }
        }

        if brackets.len() == 2 {
            // Immediately parse the whole data structure
            match dict {
                Dict::Vec(_) => {
                    let values: Vec<Dict> = s
                        .replace("[", "")
                        .replace("]", "")
                        .split(",")
                        .filter_map(|item| {
                            let Ok(d) = Dict::try_from(item.trim()) else {
                                return None;
                            };
                            Some(d)
                        })
                        .collect();
                    return Ok(Dict::Vec(values));
                }
                Dict::HashMap(_) => {
                    let pairs: HashMap<String, Dict> = s
                        .replace("{", "")
                        .replace("}", "")
                        .split(",")
                        .filter_map(|item| {
                            let Some((left, right)) = item.split_once(":") else {
                                return None;
                            };
                            let Ok(right_d) = Dict::try_from(right.trim()) else {
                                return None;
                            };
                            Some((left.replace('"', "").to_string(), right_d))
                        })
                        .collect();
                    return Ok(Dict::HashMap(pairs));
                }
                _ => return Err("Brackets detected on a primitive value"),
            }
        }

        Ok(dict)
    }
}

fn read_input(path: &str) -> Result<Dict, &str> {
    let string = fs::read_to_string(path).expect("Input file must exist");
    Dict::try_from(string.trim())
}

fn part_one(dict: &Dict) -> usize {
    0
}

fn main() {
    let dict = read_input("input.txt").expect("Bad input");
    println!("Part one: {}", part_one(&dict));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let mut s = "[1,2,3]";
        let mut d = Dict::try_from(s).expect("Bad parse!");
        println!("{:?}", d);
        assert_eq!(
            d,
            Dict::Vec(Vec::from([
                Dict::Int(1 as isize),
                Dict::Int(2 as isize),
                Dict::Int(3 as isize)
            ]))
        );
        s = "{\"a\":2,\"b\":4}";
        d = Dict::try_from(s).expect("Bad parse!");
        assert_eq!(
            d,
            Dict::HashMap(HashMap::from([
                ("a".to_string(), Dict::Int(2 as isize)),
                ("b".to_string(), Dict::Int(4 as isize)),
            ]))
        );
        s = "[[[3]]]";
        d = Dict::try_from(s).expect("Bad parse!");
        assert_eq!(
            d,
            Dict::Vec(Vec::from([Dict::Vec(Vec::from([Dict::Vec(Vec::from([
                Dict::Int(3)
            ]))]))]))
        );
    }
}
