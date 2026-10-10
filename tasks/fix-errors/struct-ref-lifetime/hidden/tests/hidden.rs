#[test]
fn parses() {
    let l = task::parse("WARN: disk almost full").unwrap();
    assert_eq!(l.level, "WARN");
    assert_eq!(l.message, "disk almost full");
    assert!(task::parse("no colon").is_none());
}
