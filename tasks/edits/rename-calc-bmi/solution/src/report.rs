use crate::metrics::{body_mass_index, category};

pub struct Person {
    pub name: String,
    pub weight_kg: f64,
    pub height_cm: f64,
}

pub fn line(p: &Person) -> String {
    let bmi = body_mass_index(p.weight_kg, p.height_cm);
    format!("{}: BMI {:.1} ({:?})", p.name, bmi, category(bmi))
}

pub fn average_bmi(people: &[Person]) -> Option<f64> {
    if people.is_empty() {
        return None;
    }
    let sum: f64 = people
        .iter()
        .map(|p| body_mass_index(p.weight_kg, p.height_cm))
        .sum();
    Some(sum / people.len() as f64)
}
