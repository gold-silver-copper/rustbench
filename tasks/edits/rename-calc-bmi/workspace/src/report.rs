use crate::metrics::{calc_bmi, category};

pub struct Person {
    pub name: String,
    pub weight_kg: f64,
    pub height_cm: f64,
}

pub fn line(p: &Person) -> String {
    let bmi = calc_bmi(p.weight_kg, p.height_cm);
    format!("{}: BMI {:.1} ({:?})", p.name, bmi, category(bmi))
}

pub fn average_bmi(people: &[Person]) -> Option<f64> {
    if people.is_empty() {
        return None;
    }
    let sum: f64 = people
        .iter()
        .map(|p| calc_bmi(p.weight_kg, p.height_cm))
        .sum();
    Some(sum / people.len() as f64)
}
