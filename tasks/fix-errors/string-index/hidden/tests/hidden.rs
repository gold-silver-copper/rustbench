#[test]
fn initials() {
    assert_eq!(task::initials("Ada Lovelace"), "AL");
    assert_eq!(task::initials("  grace   brewster hopper "), "gbh");
    assert_eq!(task::initials("Émile Zola"), "ÉZ");
    assert_eq!(task::initials(""), "");
}
