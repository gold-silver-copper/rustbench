//! Body-mass index for a fitness app.

/// BMI: weight in kilograms divided by the square of height in meters.
pub fn bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

pub fn category(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "underweight"
    } else if bmi < 25.0 {
        "normal"
    } else if bmi < 30.0 {
        "overweight"
    } else {
        "obese"
    }
}
