use task::{bmi, category};

#[test]
fn values() {
    assert!((bmi(90.0, 1.5) - 40.0).abs() < 1e-9);
    assert!((bmi(70.0, 1.75) - 22.857142857142858).abs() < 1e-9);
}

#[test]
fn categories() {
    assert_eq!(category(bmi(50.0, 1.8)), "underweight");
    assert_eq!(category(bmi(70.0, 1.75)), "normal");
    assert_eq!(category(bmi(85.0, 1.75)), "overweight");
    assert_eq!(category(bmi(90.0, 1.5)), "obese");
}
