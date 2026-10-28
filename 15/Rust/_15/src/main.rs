use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
    hash::{Hash, Hasher},
};

#[derive(Clone)]
struct Ingredient {
    name: String,
    amount: isize,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Ingredient {
    fn qualities(&self) -> Vec<isize> {
        Vec::from([
            self.amount * self.capacity,
            self.amount * self.durability,
            self.amount * self.flavor,
            self.amount * self.texture,
        ])
    }

    fn raw_qualities(&self, amount: isize) -> Vec<isize> {
        Vec::from([
            amount * self.capacity,
            amount * self.durability,
            amount * self.flavor,
            amount * self.texture,
        ])
    }
}

#[derive(Clone)]
struct Measurements {
    score: isize,
    amounts: Vec<isize>,
}

impl Measurements {
    fn score(&self, ingredients: &Vec<Ingredient>) -> isize {
        let qualities = ingredients
            .iter()
            .zip(self.amounts.iter())
            .map(|(ingredient, amount)| ingredient.raw_qualities(*amount))
            .fold(Vec::from([0, 0, 0, 0]), |mut acc, x| {
                acc.iter_mut().zip(x).for_each(|(a, i)| *a += i);
                acc
            });
        qualities
            .into_iter()
            .fold(1, |acc, x| acc * x.max(0))
            .max(0)
    }
}

impl Ord for Measurements {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Measurements {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl PartialEq for Measurements {
    fn eq(&self, other: &Measurements) -> bool {
        self.score == other.score
    }
}

impl Eq for Measurements {}

impl Hash for Measurements {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s: String = self.amounts.iter().map(|i| i.to_string() + ",").collect();
        s.hash(state)
    }
}

#[derive(Clone)]
struct Recipe {
    budget: isize,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    fn score(&self) -> isize {
        let qualities = self
            .ingredients
            .iter()
            .map(|ingredient| ingredient.qualities())
            .fold(Vec::from([0, 0, 0, 0]), |mut acc, x| {
                acc.iter_mut().zip(x).for_each(|(a, i)| *a += i);
                acc
            });
        qualities
            .into_iter()
            .fold(1, |acc, x| acc * x.max(0))
            .max(0)
    }

    fn set_amounts(&mut self, amounts: &Vec<isize>) {
        amounts.iter().enumerate().for_each(|(i, a)| {
            if i >= self.ingredients.len() {
                return;
            }
            self.ingredients[i].amount = *a;
        });
    }

    fn optimize(&mut self) -> isize {
        let mut queue = BinaryHeap::from([Measurements {
            score: 0,
            amounts: Vec::from_iter(self.ingredients.iter().map(|_| 0)),
        }]);
        let mut best_score = isize::MIN;
        let ingredients = self.ingredients.clone();
        let mut extra = 1000000;
        while !queue.is_empty() {
            let Some(measurements) = queue.pop() else {
                break;
            };

            let quantity = measurements
                .amounts
                .iter()
                .map(|a| *a)
                .fold(0, |acc, a| acc + a);
            if quantity >= self.budget && extra > 0 {
                best_score = best_score.max(measurements.score);
                extra -= 1;
                continue;
            } else if quantity >= self.budget {
                best_score = best_score.max(measurements.score);
                break;
            }
            for (i, _) in ingredients.iter().enumerate() {
                let new_amounts = measurements
                    .amounts
                    .iter()
                    .enumerate()
                    .map(|(ia, a)| {
                        if ia == i {
                            return a + 1;
                        }
                        return *a;
                    })
                    .collect();
                self.set_amounts(&new_amounts);
                let new_score = self.score();
                best_score = best_score.max(new_score);
                // println!("{:?} -> {}", new_amounts, new_score);
                queue.push(Measurements {
                    score: new_score,
                    amounts: new_amounts,
                });
            }
        }
        best_score
    }

    fn optimize_to_calories(&mut self, exact_calories: isize) -> isize {
        let mut best_score = isize::MIN;
        let mut queue = BinaryHeap::new();
        queue.push(Measurements {
            score: 0,
            amounts: Vec::from_iter(self.ingredients.iter().map(|_| 0)),
        });
        let mut visited = HashSet::new();
        while !queue.is_empty() {
            let Some(measurements) = queue.pop() else {
                continue;
            };
            let calories = measurements
                .amounts
                .iter()
                .zip(self.ingredients.iter())
                .fold(0, |a, (amount, ingredient)| {
                    a + (amount * ingredient.calories)
                });
            let total_amount: isize = measurements.amounts.iter().sum();
            // println!(
            //     "{:?} -> {} tsp {} cals -> {}",
            //     measurements.amounts, total_amount, calories, measurements.score
            // );
            if total_amount > self.budget || calories > exact_calories {
                continue;
            } else if total_amount == self.budget && calories == exact_calories {
                best_score = best_score.max(measurements.score(&self.ingredients));
                continue;
            }
            for i in 0..self.ingredients.len() {
                let mut new_amounts = measurements.amounts.clone();
                new_amounts[i] += 1;
                let mut new_measurements = Measurements {
                    score: 0,
                    amounts: new_amounts,
                };
                if visited.contains(&new_measurements) {
                    continue;
                }
                visited.insert(new_measurements.clone());
                new_measurements.score = new_measurements.score(&self.ingredients);
                queue.push(new_measurements);
            }
        }
        best_score
    }

    fn display(&self) {
        for ingredient in self.ingredients.iter() {
            println!("{}: {} tsp", ingredient.name, ingredient.amount);
        }
    }
}

fn load_input(path: &str) -> Vec<Ingredient> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.replace(":", "")
        .replace(",", "")
        .lines()
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[0].to_string();
            let capacity = parts[2].parse().expect("Bad capacity value");
            let durability = parts[4].parse().expect("Bad durability value");
            let flavor = parts[6].parse().expect("Bad flavor value");
            let texture = parts[8].parse().expect("Bad texture value");
            let calories = parts[10].parse().expect("Bad calories value");
            Ingredient {
                name,
                amount: 0,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
        .collect()
}

fn part_one(recipe: Recipe) -> isize {
    let mut recipe = recipe;
    recipe.optimize()
}

fn part_two(recipe: Recipe) -> isize {
    let mut recipe = recipe;
    recipe.optimize_to_calories(500)
}

fn main() {
    let ingredients = load_input("input.txt");
    let recipe = Recipe {
        ingredients,
        budget: 100,
    };
    println!("Part one: {}", part_one(recipe.clone()));
    println!("Part two: {}", part_two(recipe));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scoring() {
        let mut ingredients = load_input("test.txt");
        ingredients[0].amount = 44;
        ingredients[1].amount = 56;
        let mut recipe = Recipe {
            ingredients: ingredients.clone(),
            budget: 100,
        };
        assert_eq!(recipe.score(), 62842880);
        let measurements = Measurements {
            score: 0,
            amounts: ingredients.iter().map(|ing| ing.amount).collect(),
        };
        assert_eq!(measurements.score(&recipe.ingredients), 62842880);
        recipe.set_amounts(&Vec::from([1, 0]));
        assert_eq!(recipe.score(), 0);
    }

    #[test]
    fn test_part_one() {
        let mut ingredients = load_input("test.txt");
        let recipe = Recipe {
            ingredients,
            budget: 100,
        };
        assert_eq!(part_one(recipe), 62842880);
    }

    #[test]
    fn test_part_two() {
        let mut ingredients = load_input("test.txt");
        let recipe = Recipe {
            ingredients,
            budget: 100,
        };
        assert_eq!(part_two(recipe), 57600000);
    }
}
