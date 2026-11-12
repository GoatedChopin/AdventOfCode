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

const MISSILE: Spell = SPELLS[0];
const DRAIN: Spell = SPELLS[1];
const SHIELD: Spell = SPELLS[2];
const POISON: Spell = SPELLS[3];
const RECHARGE: Spell = SPELLS[4];

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

#[derive(Debug, Eq, PartialEq)]
enum TurnState {
    PlayerDeath,
    BossDeath,
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
    fn cast(&mut self, spell: Spell) -> Option<TurnState> {
        if self.other.health <= 0 {
            return Some(TurnState::BossDeath);
        }
        self.me.mana -= spell.cost;
        match spell.effect {
            SpellEffect::Damage(damage) => {
                if spell.duration == 1 {
                    self.other.health -= damage;
                }
            }
            SpellEffect::Drain(damage) => {
                self.other.health -= damage;
                self.me.health += damage;
            }
            _ => {}
        }
        self.total_mana += spell.cost;
        if spell.duration > 1 {
            self.effects.insert(spell.effect, spell.duration);
        }
        if self.other.health <= 0 {
            return Some(TurnState::BossDeath);
        }
        return None;
    }

    fn tick(&mut self) -> Option<TurnState> {
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
        if self.other.health <= 0 {
            return Some(TurnState::BossDeath);
        }
        return None;
    }

    fn boss_turn(&mut self) -> Option<TurnState> {
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
        match (self.other.health <= 0, self.me.health <= 0) {
            (true, _) => Some(TurnState::BossDeath),
            (_, true) => Some(TurnState::PlayerDeath),
            _ => None,
        }
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
        let Some(mut current_attempt) = queue.pop() else {
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
        match current_attempt.tick() {
            Some(TurnState::BossDeath) => {
                min_mana = min_mana.min(current_attempt.total_mana);
                continue;
            }
            Some(TurnState::PlayerDeath) => {
                continue;
            }
            None => {}
        }
        let spells = current_attempt.allowed_spells();
        if spells.len() == 0 {
            continue;
        }
        for s in spells {
            let mut new_attempt = current_attempt.clone();
            match new_attempt.cast(s) {
                Some(TurnState::BossDeath) => {
                    min_mana = min_mana.min(current_attempt.total_mana);
                    continue;
                }
                Some(TurnState::PlayerDeath) => {
                    continue;
                }
                None => {}
            }
            match new_attempt.tick() {
                Some(TurnState::BossDeath) => {
                    min_mana = min_mana.min(current_attempt.total_mana);
                    continue;
                }
                Some(TurnState::PlayerDeath) => {
                    continue;
                }
                None => {}
            }
            match new_attempt.boss_turn() {
                Some(TurnState::BossDeath) => {
                    min_mana = min_mana.min(current_attempt.total_mana);
                    continue;
                }
                Some(TurnState::PlayerDeath) => {
                    continue;
                }
                None => {}
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
        assert_eq!(p1, POISON.cost + MISSILE.cost)
    }

    #[test]
    fn test_poison() {
        let me = Guy {
            health: 10,
            mana: 250,
            damage: 0,
        };
        let other = Guy {
            health: 19,
            mana: 0,
            damage: 8,
        };
        let mut attempt = Attempt {
            me,
            other,
            total_mana: 0,
            turns: 0,
            effects: HashMap::new(),
        };
        attempt.cast(SPELLS[3]);
        for _ in 0..1000 {
            attempt.tick();
        }
        assert_eq!(attempt.other.health, 1);
    }

    #[test]
    fn test_scenario() {
        let me = Guy {
            health: 10,
            mana: 250,
            damage: 0,
        };
        let other = Guy {
            health: 14,
            mana: 0,
            damage: 8,
        };
        let mut attempt = Attempt {
            me,
            other,
            total_mana: 0,
            turns: 0,
            effects: HashMap::new(),
        };

        attempt.tick();
        attempt.cast(RECHARGE);

        assert_eq!(attempt.me.mana, 21);
        attempt.tick();
        attempt.boss_turn();

        assert_eq!(attempt.me.health, 2);
        assert_eq!(attempt.me.mana, 122);

        attempt.tick();
        attempt.cast(SHIELD);

        assert_eq!(attempt.me.health, 2);
        assert_eq!(attempt.me.mana, 110);

        attempt.tick();
        attempt.boss_turn();

        assert_eq!(attempt.me.health, 1);
        assert_eq!(attempt.me.mana, 211);

        attempt.tick();
        attempt.cast(DRAIN);

        assert_eq!(attempt.me.health, 3);
        assert_eq!(attempt.me.mana, 239);

        attempt.tick();
        attempt.boss_turn();

        assert_eq!(attempt.me.health, 2);
        assert_eq!(attempt.me.mana, 340);

        attempt.tick();
        attempt.cast(POISON);

        assert_eq!(attempt.other.health, 12);
        assert_eq!(attempt.me.health, 2);
        assert_eq!(attempt.me.mana, 167);

        attempt.tick();
        attempt.boss_turn();

        assert_eq!(attempt.other.health, 9);
        assert_eq!(attempt.me.health, 1);
        assert_eq!(attempt.me.mana, 167);

        attempt.tick();
        attempt.cast(MISSILE);

        assert_eq!(attempt.other.health, 2);
        assert_eq!(attempt.me.health, 1);
        assert_eq!(attempt.me.mana, 114);

        attempt.tick();
        assert_eq!(attempt.boss_turn(), Some(TurnState::BossDeath));
    }
}
