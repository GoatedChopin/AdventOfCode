use itertools::Itertools;
use std::fs;

const SHOP: [Item; 16] = [
    Item {
        cost: 8,
        bonus: Bonus::Damage(4),
        variant: ItemVariant::Weapon,
    },
    Item {
        cost: 10,
        bonus: Bonus::Damage(5),
        variant: ItemVariant::Weapon,
    },
    Item {
        cost: 25,
        bonus: Bonus::Damage(6),
        variant: ItemVariant::Weapon,
    },
    Item {
        cost: 40,
        bonus: Bonus::Damage(7),
        variant: ItemVariant::Weapon,
    },
    Item {
        cost: 74,
        bonus: Bonus::Damage(8),
        variant: ItemVariant::Weapon,
    },
    Item {
        cost: 13,
        bonus: Bonus::Armor(1),
        variant: ItemVariant::Armor,
    },
    Item {
        cost: 31,
        bonus: Bonus::Armor(2),
        variant: ItemVariant::Armor,
    },
    Item {
        cost: 53,
        bonus: Bonus::Armor(3),
        variant: ItemVariant::Armor,
    },
    Item {
        cost: 75,
        bonus: Bonus::Armor(4),
        variant: ItemVariant::Armor,
    },
    Item {
        cost: 102,
        bonus: Bonus::Armor(5),
        variant: ItemVariant::Armor,
    },
    Item {
        cost: 25,
        bonus: Bonus::Damage(1),
        variant: ItemVariant::Ring,
    },
    Item {
        cost: 50,
        bonus: Bonus::Damage(2),
        variant: ItemVariant::Ring,
    },
    Item {
        cost: 100,
        bonus: Bonus::Damage(3),
        variant: ItemVariant::Ring,
    },
    Item {
        cost: 20,
        bonus: Bonus::Armor(1),
        variant: ItemVariant::Ring,
    },
    Item {
        cost: 40,
        bonus: Bonus::Armor(2),
        variant: ItemVariant::Ring,
    },
    Item {
        cost: 80,
        bonus: Bonus::Armor(3),
        variant: ItemVariant::Ring,
    },
];

#[derive(Clone, Copy)]
enum Bonus {
    Damage(isize),
    Armor(isize),
}

#[derive(Clone, Copy)]
enum ItemVariant {
    Weapon,
    Armor,
    Ring,
}

#[derive(Clone, Copy)]
struct Item {
    cost: isize,
    bonus: Bonus,
    variant: ItemVariant,
}

impl Item {
    fn new(cost: isize, bonus: Bonus, variant: ItemVariant) -> Self {
        Self {
            cost,
            bonus,
            variant,
        }
    }
}

enum DuelOutcome {
    Stalemate,
    Victory,
    Loss,
}

#[derive(Clone, Copy)]
struct Guy {
    gold: isize,
    health: isize,
    damage: isize,
    armor: isize,
}

impl Guy {
    fn item_equip(&mut self, item: &Item) {
        self.gold -= item.cost;
        match item.bonus {
            Bonus::Armor(armor) => self.armor += armor,
            Bonus::Damage(damage) => self.damage += damage,
        }
    }

    fn item_unequip(&mut self, item: &Item) {
        self.gold += item.cost;
        match item.bonus {
            Bonus::Armor(armor) => self.armor -= armor,
            Bonus::Damage(damage) => self.damage -= damage,
        }
    }

    fn duel(&self, other: &Guy) -> DuelOutcome {
        let other_effective_damage = (other.damage - self.armor).max(1);
        let me_effective_damage = (self.damage - other.armor).max(1);
        if other_effective_damage <= 0 && me_effective_damage <= 0 {
            return DuelOutcome::Stalemate;
        }
        let my_turns = match other_effective_damage <= 0 {
            true => isize::MAX,
            false => self.health / other_effective_damage,
        };
        let other_turns = match me_effective_damage <= 0 {
            true => isize::MAX,
            false => other.health / me_effective_damage,
        };

        match my_turns >= other_turns {
            true => DuelOutcome::Victory,
            false => DuelOutcome::Loss,
        }
    }
}

fn read_input(path: &str) -> Guy {
    let s = fs::read_to_string(path).expect("Input file must exist");
    let attributes: Vec<isize> = s
        .lines()
        .map(|line| {
            let (_, value) = line
                .trim()
                .split_once(": ")
                .expect("Line must be <attribute>: <value>");
            value.parse().expect("Bad attribute value")
        })
        .collect();
    Guy {
        gold: 0,
        health: attributes[0],
        damage: attributes[1],
        armor: attributes[2],
    }
}

fn valid_items(items: &Vec<&Item>) -> bool {
    let mut rings = 0;
    let mut weapons = 0;
    let mut armor = 0;
    for item in items.iter() {
        match item.variant {
            ItemVariant::Armor => armor += 1,
            ItemVariant::Ring => rings += 1,
            ItemVariant::Weapon => weapons += 1,
        }
    }
    rings <= 2 && armor <= 1 && weapons == 1
}

fn part_one(me: Guy, enemy: Guy) -> Option<isize> {
    // returns None if you cannot beat the boss
    let mut me = me.clone();
    let mut lowest_budget: Option<isize> = None;
    for num_items in 0..=4 {
        for combo in SHOP.iter().combinations(num_items) {
            if !valid_items(&combo) {
                continue;
            }
            let combo_cost = combo.iter().map(|item| item.cost).sum();
            match lowest_budget {
                Some(lb) => {
                    if lb < combo_cost {
                        continue;
                    }
                }
                _ => {}
            }
            combo.iter().for_each(|item| {
                me.item_equip(*item);
            });

            match me.duel(&enemy) {
                DuelOutcome::Victory => match lowest_budget {
                    Some(lb) => lowest_budget = Some(lb.min(combo_cost)),
                    None => lowest_budget = Some(combo_cost),
                },
                _ => {}
            }

            combo.iter().for_each(|item| {
                me.item_unequip(*item);
            });
        }
    }

    lowest_budget
}

fn part_two(me: Guy, enemy: Guy) -> Option<isize> {
    // returns None if you cannot beat the boss
    let mut me = me.clone();
    let mut max_budget: Option<isize> = None;
    for num_items in 0..=4 {
        for combo in SHOP.iter().combinations(num_items) {
            if !valid_items(&combo) {
                continue;
            }
            let combo_cost = combo.iter().map(|item| item.cost).sum();
            match max_budget {
                Some(mb) => {
                    if mb > combo_cost {
                        continue;
                    }
                }
                _ => {}
            }
            combo.iter().for_each(|item| {
                me.item_equip(*item);
            });

            match me.duel(&enemy) {
                DuelOutcome::Loss => match max_budget {
                    Some(mb) => max_budget = Some(mb.max(combo_cost)),
                    None => max_budget = Some(combo_cost),
                },
                _ => {}
            }

            combo.iter().for_each(|item| {
                me.item_unequip(*item);
            });
        }
    }

    max_budget
}

fn main() {
    let me = Guy {
        gold: 0,
        health: 100,
        damage: 0,
        armor: 0,
    };
    let enemy = read_input("input.txt");
    println!(
        "Part one: {}",
        part_one(me, enemy).expect("Enemy must be defeatable")
    );
    println!(
        "Part two: {}",
        part_two(me, enemy).expect("Enemy must be loseable")
    );
}
