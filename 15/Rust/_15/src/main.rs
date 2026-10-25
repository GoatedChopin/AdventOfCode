use std::fs;

struct Ingredient {
    amount: isize,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
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
}

fn load_input(path: &str) -> Vec<Ingredient> {
    Vec::new()
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
        let recipe = Recipe { ingredients };
        assert_eq!(recipe.score(), 62842880);
    }
}
