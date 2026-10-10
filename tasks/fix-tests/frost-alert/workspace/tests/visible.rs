use task::frost_hours;

#[test]
fn zero_degrees_is_frost() {
    assert_eq!(frost_hours(&[3.5, 0.0, -1.2, 2.0]), vec![1, 2]);
}
