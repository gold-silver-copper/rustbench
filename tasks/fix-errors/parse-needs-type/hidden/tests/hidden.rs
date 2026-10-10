#[test]
fn doubles() {
    assert_eq!(task::doubled(" 21 ").as_deref(), Some("21 x2 = 42"));
    assert_eq!(task::doubled("0").as_deref(), Some("0 x2 = 0"));
    assert_eq!(task::doubled("-3"), None);
    assert_eq!(task::doubled("2.5"), None);
    assert_eq!(task::doubled("lots"), None);
}
