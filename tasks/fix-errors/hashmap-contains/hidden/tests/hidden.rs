use std::collections::HashMap;

#[test]
fn registers() {
    let mut u = HashMap::new();
    assert!(task::register(&mut u, "kim", 1));
    assert!(!task::register(&mut u, "kim", 2));
    assert_eq!(u.get("kim"), Some(&1));
}
