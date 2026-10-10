use task::seed::starter_recipes;
use task::{Ingredient, Recipe};

#[test]
fn new_defaults_to_one() {
    let r = Recipe::new("soup").add("leek", 200).add("potato", 300);
    assert_eq!(r.servings, 1);
    assert_eq!(r.ingredient_count(), 2);
    assert_eq!(r.total_grams(), 500);
}

#[test]
fn literal_with_servings() {
    let r = Recipe {
        name: "stew".into(),
        ingredients: vec![Ingredient { name: "beef".into(), grams: 400 }],
        minutes: 120,
        servings: 4,
    };
    assert_eq!(r.servings, 4);
    assert_eq!(r.ingredient_count(), 1);
}

#[test]
fn seed_data_is_one_serving() {
    let book = starter_recipes();
    assert_eq!(book.len(), 3);
    assert!(book.iter().all(|r| r.servings == 1));
    assert_eq!(book[0].ingredient_count(), 3);
    assert_eq!(book[0].minutes, 20);
}
