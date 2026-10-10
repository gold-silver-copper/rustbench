use crate::recipe::{Ingredient, Recipe};

fn ing(name: &str, grams: u32) -> Ingredient {
    Ingredient {
        name: name.to_string(),
        grams,
    }
}

/// The recipes every new cookbook starts with.
pub fn starter_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            name: "pancakes".to_string(),
            ingredients: vec![ing("flour", 125), ing("milk", 300), ing("egg", 50)],
            minutes: 20,
        },
        Recipe {
            name: "porridge".to_string(),
            ingredients: vec![ing("oats", 40), ing("milk", 250)],
            minutes: 8,
        },
        Recipe::new("toast").add("bread", 60).add("butter", 10),
    ]
}
