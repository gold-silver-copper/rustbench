#[test]
fn passing() {
    assert_eq!(task::passing(&[40, 75, 50, 90], 50), vec![75, 50, 90]);
    assert!(task::passing(&[1, 2], 50).is_empty());
}
