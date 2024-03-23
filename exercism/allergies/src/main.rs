// source: https://exercism.org/tracks/rust/exercises/allergies

// slight modification: each person is allergic to a list of items and in a certain degree.
// given a dish, which have certain ingredients in certain quantitities (grams for example), the program must tell how much a person is allergic to that dish in all the ways judged apt:
// for example could be good on an absolute term in relation to the ingredients' quantities, or in a proportional term in relation to the ingredients' proportions

use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    allergies: HashMap<String, u32>,
}

#[derive(Debug)]
struct Dish {
    name: String,
    ingredients: HashMap<String, u32>,
}

type Percentage01 = f64;

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            allergies: HashMap::new(),
        }
    }

    fn set_allergy(&mut self, food: &str, allergy_degree: u32) {
        self.allergies.insert(food.to_string(), allergy_degree);
    }

    fn get_allergy_to(&self, food: &str) -> Option<u32> {
        self.allergies.get(food).copied()
    }

    fn get_total_allergic_score(&self) -> u32 {
        self.allergies.values().sum()
    }

    fn get_allergic_score_to(&self, dish: &Dish) -> u32 {
        let mut score: u32 = 0;
        for (ingredient, grams) in &dish.ingredients {
            score += self.get_allergy_to(ingredient).unwrap_or(0) * grams;
        }
        score
    }

    fn get_proportional_allergic_score_to(&self, dish: &Dish) -> f64 {
        let mut score: f64 = 0.0;
        for (ingredient, percentage01) in dish.proportions() {
            score += (self.get_allergy_to(&ingredient).unwrap_or(0) as f64) * percentage01;
        }
        score
    }
}

impl Dish {
    fn new(name: &str) -> Dish {
        Dish {
            name: name.to_string(),
            ingredients: HashMap::new(),
        }
    }

    fn set_ingredient(&mut self, food: &str, grams: u32) {
        self.ingredients.insert(food.to_string(), grams);
    }

    fn get_amount_of(&self, food: &str) -> Option<u32> {
        self.ingredients.get(food).copied()
    }

    fn weight(&self) -> u32 {
        self.ingredients.values().sum()
    }

    fn proportions(&self) -> HashMap<String, Percentage01> {
        let mut proportions: HashMap<String, Percentage01> = HashMap::new();
        let total = self.weight() as f64;

        for (ingredient, ingredient_grams) in &self.ingredients {
            proportions.insert(ingredient.clone(), (*ingredient_grams as f64) / total);
        }
        proportions
    }
}

fn main() {
    let mut mike = Person::new("Mike");
    mike.set_allergy("eggs", 1);
    mike.set_allergy("kryptonite", 600);
    mike.set_allergy("frogs", 2);
    mike.set_allergy("maggots", 10);
    mike.set_allergy("bugs", 10);
    mike.set_allergy("polonium", 6);

    let mut strange_borscht = Dish::new("strange Borscht");
    strange_borscht.set_ingredient("cebula", 60);
    strange_borscht.set_ingredient("polonium", 5);
    strange_borscht.set_ingredient("eggs", 5);
    strange_borscht.set_ingredient("frogs", 1);
    strange_borscht.set_ingredient("kryptonite", 1);

    println!("We have {:?} and {:?}", mike, strange_borscht);

    println!(
        "{} has an absolute allergy score to {} of {}",
        mike.name,
        strange_borscht.name,
        mike.get_allergic_score_to(&strange_borscht)
    );

    println!("let us compare the different allergy scores:\nthe absolute one of {} is {}, while his allergic score to {} is {}, and his proportional allergic score to it is {}", mike.name, mike.get_total_allergic_score(), strange_borscht.name, mike.get_allergic_score_to(&strange_borscht), mike.get_proportional_allergic_score_to(&strange_borscht));
}

#[cfg(test)]
mod test {
    #[test]
    fn check_allergy() {}
}
