/// Body mass index: weight in kg divided by height in metres squared.
pub fn body_mass_index(weight_kg: f64, height_cm: f64) -> f64 {
    let m = height_cm / 100.0;
    weight_kg / (m * m)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Under,
    Normal,
    Over,
    Obese,
}

pub fn category(bmi: f64) -> Category {
    if bmi < 18.5 {
        Category::Under
    } else if bmi < 25.0 {
        Category::Normal
    } else if bmi < 30.0 {
        Category::Over
    } else {
        Category::Obese
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_value() {
        let b = body_mass_index(70.0, 175.0);
        assert!((b - 22.857).abs() < 0.001);
        assert_eq!(category(b), Category::Normal);
    }
}
