#[test]
fn filters() {
    let v = vec!["ok_name".to_string(), "x".into(), "bad name".into(), "abc".into()];
    assert_eq!(task::accepted(v), vec!["ok_name", "abc"]);
}
