#[test]
fn values() {
    assert_eq!(task::value("retries=3"), Some(3));
    assert_eq!(task::value("offset = -12"), Some(-12));
    assert_eq!(task::value("retries"), None);
    assert_eq!(task::value("retries=many"), None);
}
