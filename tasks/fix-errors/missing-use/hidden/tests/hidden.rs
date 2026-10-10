#[test]
fn counts_words() {
    let c = task::word_counts("the cat and The dog");
    assert_eq!(c.get("the"), Some(&2));
    assert_eq!(c.get("dog"), Some(&1));
    assert_eq!(c.len(), 4);
}
