#[test]
fn averages() {
    assert_eq!(task::average(&[4, 5, 3, 4]), 4.0);
    assert_eq!(task::average(&[1, 2]), 1.5);
    assert_eq!(task::average(&[]), 0.0);
}
