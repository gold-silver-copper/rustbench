#[test]
fn names() {
    assert_eq!(task::roster_name("Ada", "Lovelace"), "Lovelace, Ada");
    assert_eq!(task::roster_name("", "Solo"), "Solo, ");
}
