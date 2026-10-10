use task::longest_word;

#[test]
fn longest() {
    assert_eq!(longest_word("one three five"), Some("three"));
    assert_eq!(longest_word("ab cd ef"), Some("ab"));
    assert_eq!(longest_word("\tsolo\n"), Some("solo"));
    assert_eq!(longest_word("ééé abcd"), Some("abcd"));
    assert_eq!(longest_word("abc ééé"), Some("abc"));
}

#[test]
fn no_words() {
    assert_eq!(longest_word(""), None);
    assert_eq!(longest_word(" \n\t"), None);
}
