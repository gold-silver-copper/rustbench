use task::seed::starter_recipes;
use task::{Ingredient, Recipe};

fn main() {
    let mut book = starter_recipes();
    book.push(Recipe {
        name: "salad".to_string(),
        ingredients: vec![Ingredient {
            name: "lettuce".to_string(),
            grams: 150,
        }],
        minutes: 5,
    });
    for r in &book {
        println!("{}: {} ingredients, {} g", r.name, r.ingredient_count(), r.total_grams());
    }
}
