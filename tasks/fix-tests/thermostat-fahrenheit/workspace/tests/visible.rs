use task::c_to_f;

#[test]
fn boiling_point_in_fahrenheit() {
    assert_eq!(c_to_f(100), 212);
}
