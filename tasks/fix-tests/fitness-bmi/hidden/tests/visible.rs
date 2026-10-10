use task::bmi;

#[test]
fn bmi_of_tall_person() {
    assert!((bmi(80.0, 2.0) - 20.0).abs() < 1e-9);
}
