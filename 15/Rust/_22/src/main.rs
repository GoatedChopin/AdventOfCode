use std::cmp::{Ord, Reverse};
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Guy {
    health: isize,
    mana: isize,
    damage: isize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

#[derive(Clone)]
struct Attempt {
    total_mana: isize,
    turns: usize,
    me: Guy,
    other: Guy,
    effects: HashMap<SpellEffect, isize>,
}

impl Attempt {
    fn cast(&mut self, spell: Spell) {
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
    }

    fn tick(&mut self) {
        self.turns += 1;
        self.effects = self
            .effects
            .iter()
            .filter_map(|(effect, duration)| {
                if *duration == 1 {
                    return None;
                }
                Some((*effect, duration - 1))
            })
            .collect();
        let mut damage = self.other.damage;
        self.effects.iter().for_each(|(effect, _)| {
            match effect {
                SpellEffect::Damage(damage) => self.other.health -= damage,
                SpellEffect::Recharge(mana) => self.me.mana += mana,
                SpellEffect::Shield(armor) => damage = (damage - armor).max(1),
                _ => {}
            };
        });
        self.me.health -= damage;
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
        for s in spells {
            let mut new_attempt = current_attempt.clone();
            new_attempt.cast(s);
            new_attempt.tick();
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
