use task::{mean, range};

#[test]
fn means() {
    assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
    assert_eq!(mean(&[-4.0]), Some(-4.0));
    let none: Vec<f64> = Vec::new();
    assert_eq!(mean(&none), None);
}

#[test]
fn ranges() {
    assert_eq!(range(&[3.0, -1.0, 2.0]), Some(4.0));
}
