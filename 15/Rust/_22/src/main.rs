use std::cmp::Ord;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

const SPELLS: [Spell; 5] = [
    Spell {
        cost: 53,
        effect: SpellEffect::Damage(4),
        duration: 1,
    },
    Spell {
        cost: 73,
        effect: SpellEffect::Drain(2),
        duration: 1,
    },
    Spell {
        cost: 113,
        effect: SpellEffect::Shield(7),
        duration: 6,
    },
    Spell {
        cost: 173,
        effect: SpellEffect::Damage(3),
        duration: 6,
    },
    Spell {
        cost: 229,
        effect: SpellEffect::Recharge(101),
        duration: 5,
    },
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Guy {
    health: isize,
    mana: isize,
    damage: isize,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum SpellEffect {
    Damage(isize),
    Drain(isize),
    Recharge(isize),
    Shield(isize),
}

#[derive(Copy, Clone)]
struct Spell {
    cost: isize,
    effect: SpellEffect,
    duration: isize,
}

fn read_input(path: &str) -> Guy {
    let s = fs::read_to_string(path).expect("input.txt must exist");
    let attributes: Vec<isize> = s
        .lines()
        .map(|line| {
            let (_, right) = line.split_once(": ").expect("Bad input line");
            right.parse().expect("Bad attribute value")
        })
        .collect();
    Guy {
        health: attributes[0],
        mana: 0,
        damage: attributes[1],
    }
}

#[derive(Clone, Debug)]
struct Attempt {
    total_mana: isize,
    turns: usize,
    me: Guy,
    other: Guy,
    effects: HashMap<SpellEffect, isize>,
}

impl Attempt {
    fn cast(&mut self, spell: Spell) -> bool {
        if self.other.health <= 0 {
            return true;
        }
        self.me.mana -= spell.cost;
        match spell.effect {
            SpellEffect::Damage(damage) => self.other.health -= damage,
            SpellEffect::Drain(damage) => {
                self.other.health -= damage;
                self.me.health += damage;
            }
            SpellEffect::Recharge(mana) => {
                self.me.mana += mana;
            }
            _ => {}
        }
        self.total_mana += spell.cost;
        if spell.duration > 1 {
            self.effects.insert(spell.effect, spell.duration);
        }
        return self.other.health <= 0;
    }

    fn tick(&mut self) -> bool {
        self.turns += 1;
        self.effects = self
            .effects
            .iter()
            .filter_map(|(effect, duration)| {
                if *duration <= 0 {
                    return None;
                }
                Some((*effect, duration - 1))
            })
            .collect();
        self.effects.iter().for_each(|(effect, _)| {
            match effect {
                SpellEffect::Damage(damage) => self.other.health -= damage,
                SpellEffect::Recharge(mana) => self.me.mana += mana,
                _ => {}
            };
        });
        return self.other.health <= 0;
    }

    fn boss_turn(&mut self) -> bool {
        let mut damage = self.other.damage;
        self.effects.iter().for_each(|(effect, _)| {
            match effect {
                SpellEffect::Shield(armor) => damage = (damage - armor).max(1),
                _ => {}
            };
        });
        if self.other.health > 0 {
            self.me.health -= damage;
        }
        return self.other.health <= 0;
    }

    fn allowed_spells(&self) -> Vec<Spell> {
        SPELLS
            .iter()
            .filter(|&&spell| {
                if spell.cost > self.me.mana {
                    return false;
                }
                if self.effects.contains_key(&spell.effect) {
                    return false;
                }
                return true;
            })
            .map(|spell| *spell)
            .collect()
    }
}

impl PartialEq for Attempt {
    fn eq(&self, other: &Attempt) -> bool {
        self.total_mana > other.total_mana
    }
}

impl Eq for Attempt {}

impl PartialOrd for Attempt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.total_mana.cmp(&other.total_mana))
    }
}

impl Ord for Attempt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_mana.cmp(&other.total_mana)
    }
}

fn part_one(me: Guy, other: Guy) -> isize {
    let mut min_mana = isize::MAX;
    let mut queue = BinaryHeap::from([Attempt {
        total_mana: 0,
        turns: 0,
        me,
        other,
        effects: HashMap::new(),
    }]);
    while !queue.is_empty() {
        let Some(current_attempt) = queue.pop() else {
            continue;
        };
        println!("{:?}", current_attempt);
        if current_attempt.total_mana > min_mana {
            continue;
        }
        if current_attempt.me.health <= 0 {
            continue;
        }
        if current_attempt.other.health <= 0 {
            min_mana = min_mana.min(current_attempt.total_mana);
            continue;
        }
        let spells = current_attempt.allowed_spells();
        if spells.len() == 0 {
            continue;
        }
        for s in spells {
            let mut new_attempt = current_attempt.clone();
            if new_attempt.tick() {
                min_mana = min_mana.min(new_attempt.total_mana);
                break;
            }
            if new_attempt.cast(s) {
                min_mana = min_mana.min(new_attempt.total_mana);
                break;
            }
            if new_attempt.tick() {
                min_mana = min_mana.min(new_attempt.total_mana);
                break;
            }
            if new_attempt.boss_turn() {
                min_mana = min_mana.min(new_attempt.total_mana);
                break;
            }
            queue.push(new_attempt)
        }
    }

    min_mana
}

fn main() {
    let me = Guy {
        health: 50,
        mana: 500,
        damage: 0,
    };
    let boss = read_input("input.txt");
    println!("Part one: {}", part_one(me, boss));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let me = Guy {
            health: 10,
            mana: 250,
            damage: 0,
        };
        let other = Guy {
            health: 13,
            mana: 0,
            damage: 8,
        };
        let p1 = part_one(me, other);
        assert_eq!(p1, 173 + 53);
        let other = Guy {
            health: 14,
            mana: 0,
            damage: 8,
        };
        let p1 = part_one(me, other);
        println!("Harder");
        assert_eq!(p1, 229 + 113 + 73 + 173 + 53)
    }
}
