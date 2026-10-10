use task::first_unbalanced;

#[test]
fn balanced() {
    assert_eq!(first_unbalanced(""), None);
    assert_eq!(first_unbalanced("no brackets"), None);
    assert_eq!(first_unbalanced("{[()()]}[]"), None);
}

#[test]
fn bad_closer() {
    assert_eq!(first_unbalanced(")"), Some(0));
    assert_eq!(first_unbalanced("a)b("), Some(1));
    assert_eq!(first_unbalanced("([)]"), Some(2));
    assert_eq!(first_unbalanced("{(})"), Some(2));
    assert_eq!(first_unbalanced("((( ]"), Some(4));
}

#[test]
fn never_closed() {
    assert_eq!(first_unbalanced("(()"), Some(0));
    assert_eq!(first_unbalanced("x[{}(()"), Some(1));
    assert_eq!(first_unbalanced("()("), Some(2));
}

#[test]
fn counts_chars_not_bytes() {
    assert_eq!(first_unbalanced("ééé)"), Some(3));
    assert_eq!(first_unbalanced("日本[語"), Some(2));
}
