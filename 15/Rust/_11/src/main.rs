use std::collections::HashSet;
use std::fs;

enum RuleVariant {
    CannotContain(char),
}

struct Rule {
    variant: RuleVariant,
}

impl Rule {
    fn new(variant: RuleVariant) -> Self {
        Self { variant }
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

    fn set_default_rules(&mut self) {
        self.rules = Vec::from([
            Rule::new(RuleVariant::CannotContain('i')),
            Rule::new(RuleVariant::CannotContain('o')),
            Rule::new(RuleVariant::CannotContain('l')),
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
}
