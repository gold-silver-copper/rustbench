#[test]
fn converts() {
    assert_eq!(task::to_dollars(&[150, 5, 0]), vec![1.5, 0.05, 0.0]);
}
