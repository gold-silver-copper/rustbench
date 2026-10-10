#[test]
fn low_stock() {
    assert_eq!(task::below_threshold(&[("a", 1), ("b", 5)], 5), vec!["a".to_string()]);
    assert!(task::below_threshold(&[], 5).is_empty());
}
