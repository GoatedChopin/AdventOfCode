use std::{collections::BinaryHeap, fs};

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
}

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
        qualities.into_iter().fold(1, |acc, x| acc * x).max(0)
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
        let mut queue = BinaryHeap::from([(0, Vec::from_iter(self.ingredients.iter().map(|_| 0)))]);
        let mut best_score = isize::MIN;
        let ingredients = self.ingredients.clone();
        while !queue.is_empty() {
            let Some((current_score, current_amounts)) = queue.pop() else {
                break;
            };
            if current_amounts.iter().map(|a| *a).fold(0, |acc, a| acc + a) > self.budget {
                break;
            }
            for (i, ing) in ingredients.iter().enumerate() {
                let new_amounts = current_amounts
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
                queue.push((new_score, new_amounts));
            }
        }
        best_score
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
    recipe.optimize();
    recipe.score()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scoring() {
        let mut ingredients = load_input("test.txt");
        ingredients[0].amount = 44;
        ingredients[1].amount = 56;
        let recipe = Recipe {
            ingredients,
            budget: 100,
        };
        assert_eq!(recipe.score(), 62842880);
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
}
