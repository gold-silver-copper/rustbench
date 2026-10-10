#[test]
fn splits() {
    assert_eq!(task::fields("Ada, 36, LONDON"), vec!["ada", "36", "london"]);
    assert_eq!(task::fields("x"), vec!["x"]);
}
