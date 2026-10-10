#[derive(Debug, Clone, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub grams: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub minutes: u32,
}

impl Recipe {
    pub fn new(name: &str) -> Self {
        Recipe {
            name: name.to_string(),
            ingredients: Vec::new(),
            minutes: 0,
        }
    }

    pub fn add(mut self, name: &str, grams: u32) -> Self {
        self.ingredients.push(Ingredient {
            name: name.to_string(),
            grams,
        });
        self
    }

    pub fn ingredient_count(&self) -> usize {
        self.ingredients.len()
    }

    pub fn total_grams(&self) -> u32 {
        self.ingredients.iter().map(|i| i.grams).sum()
    }
}
