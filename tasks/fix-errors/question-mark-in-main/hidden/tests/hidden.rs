#[test]
fn totals() {
    assert_eq!(task::parse_total("1, 2,3"), Ok(6));
    assert!(task::parse_total("1, x").is_err());
}
