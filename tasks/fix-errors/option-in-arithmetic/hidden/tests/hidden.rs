use std::collections::HashMap;

#[test]
fn costs() {
    let mut p = HashMap::new();
    p.insert("apple".to_string(), 40);
    assert_eq!(task::cost(&p, "apple", 3), Some(120));
    assert_eq!(task::cost(&p, "pear", 3), None);
}
