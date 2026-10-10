#[test]
fn counts() {
    assert_eq!(task::vowels("Queue"), 4);
    assert_eq!(task::vowels("rhythm"), 0);
    assert_eq!(task::vowels("café au lait"), 5);
}
