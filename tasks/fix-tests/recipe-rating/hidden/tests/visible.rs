use task::label;

#[test]
fn shows_rounded_average() {
    assert_eq!(label(&[5, 5, 4]), "4.7 stars");
}
