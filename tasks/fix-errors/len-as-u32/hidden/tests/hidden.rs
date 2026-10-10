#[test]
fn counts() {
    assert_eq!(task::word_count("hi there  you"), 3);
    assert_eq!(task::word_count(""), 0);
}
