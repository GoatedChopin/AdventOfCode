use std::collections::HashSet;
use std::fs;

use regex::{Regex, regex};

use crate::RuleVariant::CannotContain;

enum RuleVariant {
    CannotContain(char),
    Pattern(String),
}

struct Rule {
    variant: RuleVariant,
    count: usize,
}

impl Rule {
    fn new(variant: RuleVariant, count: usize) -> Self {
        Self { variant, count }
    }

    fn passes(&self, s: &String) -> bool {
        use RuleVariant::*;
        match &self.variant {
            CannotContain(p) => !s.contains(*p),
            Pattern(p) => {
                let Ok(re) = Regex::new(p) else {
                    return false;
                };
                let Some(captures) = re.captures(&s) else {
                    return false;
                };
                let mut valid_captures = Vec::new();
                for capture_option in captures.iter() {
                    let Some(capture) = capture_option else {
                        continue;
                    };
                    if valid_captures.is_empty() {
                        valid_captures.push(capture);
                        continue;
                    }
                    if valid_captures[valid_captures.len() - 1].end() < capture.start() {
                        valid_captures.push(capture);
                        continue;
                    }
                }
                return valid_captures.len() >= self.count;
            }
        }
    }
}

struct Password {
    inner: String,
    rules: Vec<Rule>,
}

impl Password {
    fn increment(&mut self) {
        let cannot_contain_char_indices: HashSet<i32> = self
            .rules
            .iter()
            .filter_map(|r| match &r.variant {
                RuleVariant::CannotContain(c) => Some(*c as i32 - 'a' as i32),
                RuleVariant::Pattern(_) => None,
            })
            .collect();
        let new_chars: Vec<char> = self.inner.clone().chars().collect();
        let mut char_indices: Vec<i32> = new_chars
            .into_iter()
            .map(|c| c as i32 - 'a' as i32)
            .collect();
        let mut i = char_indices.len() - 1;
        while i >= 0 {
            let mut ci = char_indices[i];
            while ci < 25 && (char_indices[i] == ci || cannot_contain_char_indices.contains(&ci)) {
                ci += 1;
            }
            if char_indices[i] != ci {
                char_indices[i] = ci;
                break;
            }
            if ci > 25 {
                char_indices[i] = 0;
            }
            if i > 0 {
                i = i - 1;
            } else {
                break;
            }
        }
        for after in (i + 1)..char_indices.len() {
            char_indices[after] = 0;
        }
        let new_string: String = char_indices
            .into_iter()
            .map(|ci| (ci + ('a' as i32)) as u8 as char)
            .collect();
        self.inner = new_string;
    }

    /// Assumes the default rules
    fn is_valid_indices(&self, char_indices: &Vec<i32>) -> bool {
        let mut ascending = false;
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        let mut i = 0;
        while (!ascending || pairs.len() < 2) && i < char_indices.len() - 1 {
            let j = i + 1;
            if j - i == 1 && char_indices[i] == char_indices[j] {
                if pairs.len() > 0 && pairs[pairs.len() - 1].1 != i {
                    pairs.push((i, j));
                } else if pairs.len() == 0 {
                    pairs.push((i, j));
                }
            }
            let can_ascend = char_indices[j] - char_indices[i] == 1;
            if can_ascend
                && j + 1 < char_indices.len()
                && char_indices[j + 1] == char_indices[j] + 1
            {
                ascending = true;
            }
            i += 1;
        }
        ascending && pairs.len() >= 2
    }

    fn find_next_valid(&mut self) {
        let cannot_contain_char_indices: HashSet<i32> = self
            .rules
            .iter()
            .filter_map(|r| match &r.variant {
                RuleVariant::CannotContain(c) => Some(*c as i32 - 'a' as i32),
                RuleVariant::Pattern(_) => None,
            })
            .collect();
        let new_chars: Vec<char> = self.inner.clone().chars().collect();
        let mut char_indices: Vec<i32> = new_chars
            .into_iter()
            .map(|c| c as i32 - 'a' as i32)
            .collect();
        let mut iterations = 0;
        while !self.is_valid_indices(&char_indices) {
            let mut i = char_indices.len() - 1;
            loop {
                let mut ci = char_indices[i];
                while ci < 25
                    && (char_indices[i] == ci || cannot_contain_char_indices.contains(&ci))
                {
                    ci += 1;
                }
                if char_indices[i] != ci {
                    char_indices[i] = ci;
                    break;
                }
                if ci > 25 {
                    char_indices[i] = 0;
                }
                if i > 0 {
                    i = i - 1;
                } else {
                    break;
                }
                if iterations % 100000 == 0 {
                    println!(
                        "Current password: {:?} Iterations: {}",
                        char_indices, iterations
                    );
                }
                iterations += 1;
            }
            for after in (i + 1)..char_indices.len() {
                char_indices[after] = 0;
            }
        }
        let new_string: String = char_indices
            .into_iter()
            .map(|ci| (ci + ('a' as i32)) as u8 as char)
            .collect();
        self.inner = new_string;
    }

    fn is_valid(&self) -> bool {
        self.rules.iter().all(|r| r.passes(&self.inner))
    }

    fn set_default_rules(&mut self) {
        self.rules = Vec::from([
            Rule::new(RuleVariant::CannotContain('i'), 0),
            Rule::new(RuleVariant::CannotContain('o'), 0),
            Rule::new(RuleVariant::CannotContain('l'), 0),
            Rule::new(RuleVariant::Pattern("([a-z])\\1".to_string()), 2),
            Rule::new(RuleVariant::Pattern("(?:(?:a(?=b)|b(?=c)|c(?=d)|d(?=e)|e(?=f)|f(?=g)|g(?=h)|h(?=i)|i(?=j)|j(?=k)|k(?=l)|l(?=m)|m(?=n)|n(?=o)|o(?=p)|p(?=q)|q(?=r)|r(?=s)|s(?=t)|t(?=u)|u(?=v)|v(?=w)|w(?=x)|x(?=y)|y(?=z)){2,}[a-z])".to_string()), 1),
        ]);
    }
}

impl Into<Password> for String {
    fn into(self) -> Password {
        Password {
            inner: self,
            rules: Vec::new(),
        }
    }
}

fn read_input(f: &str) -> String {
    fs::read_to_string(f)
        .expect("Input file must exist")
        .trim()
        .to_string()
}

fn main() {
    let mut input: Password = read_input("input.txt").into();
    input.set_default_rules();
    input.find_next_valid();
    println!("Part one: {}", input.inner);
    input.increment();
    input.find_next_valid();
    println!("Part two: {}", input.inner);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_char_digits() {
        let alphabet: Vec<char> = ('a'..='z').collect();
        println!("alphabet len {}", alphabet.len());
        for (i, &c) in alphabet.iter().enumerate() {
            let ch_num = c as i32 - 'a' as i32;
            assert_eq!(i as i32, ch_num);
        }
    }

    #[test]
    fn test_password_increment() {
        let mut password: Password = "aaaaa".to_string().into();
        password.increment();
        assert_eq!(password.inner, "aaaab".to_string());
        password.increment();
        assert_eq!(password.inner, "aaaac".to_string());
        password.inner = "aazzzz".to_string();
        password.increment();
        assert_eq!(password.inner, "abaaaa".to_string());
    }

    #[test]
    fn test_part_one() {
        let mut password: Password = "abcdefgh".to_string().into();
        password.set_default_rules();
        password.find_next_valid();
        assert_eq!(password.inner, "abcdffaa".to_string());
        password = "ghijklmn".to_string().into();
        password.set_default_rules();
        password.find_next_valid();
        assert_eq!(password.inner, "ghjaabcc".to_string());
    }
}
