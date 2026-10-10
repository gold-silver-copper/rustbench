use task::{mean, range};

#[test]
fn no_readings_no_mean() {
    assert_eq!(mean(&[]), None);
    assert_eq!(range(&[]), None);
}
