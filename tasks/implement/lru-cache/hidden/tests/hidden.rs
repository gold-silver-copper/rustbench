use task::LruCache;

fn ev(k: &str, v: i64) -> Option<(String, i64)> {
    Some((k.to_string(), v))
}

#[test]
fn evicts_least_recently_used() {
    let mut c = LruCache::new(3);
    assert!(c.is_empty());
    assert_eq!(c.put("a", 1), None);
    assert_eq!(c.put("b", 2), None);
    assert_eq!(c.put("c", 3), None);
    assert_eq!(c.get("a"), Some(1));
    assert_eq!(c.put("d", 4), ev("b", 2));
    assert_eq!(c.put("e", 5), ev("c", 3));
    assert_eq!(c.put("f", 6), ev("a", 1));
    assert_eq!(c.len(), 3);
    assert!(!c.is_empty());
    assert_eq!(c.get("a"), None);
    assert_eq!(c.get("b"), None);
}

#[test]
fn replacing_is_a_use_and_evicts_nothing() {
    let mut c = LruCache::new(2);
    c.put("a", 1);
    c.put("b", 2);
    assert_eq!(c.put("a", 10), None);
    assert_eq!(c.len(), 2);
    assert_eq!(c.put("c", 3), ev("b", 2));
    assert_eq!(c.peek("a"), Some(10));
}

#[test]
fn peek_and_contains_do_not_use() {
    let mut c = LruCache::new(2);
    c.put("a", 1);
    c.put("b", 2);
    assert_eq!(c.peek("a"), Some(1));
    assert!(c.contains("a"));
    assert!(!c.contains("z"));
    assert_eq!(c.peek("z"), None);
    assert_eq!(c.put("c", 3), ev("a", 1));
    assert!(!c.contains("a"));
}

#[test]
fn missing_get_changes_nothing() {
    let mut c = LruCache::new(2);
    c.put("a", 1);
    c.put("b", 2);
    assert_eq!(c.get("x"), None);
    assert_eq!(c.len(), 2);
    assert_eq!(c.put("c", 3), ev("a", 1));
}

#[test]
fn capacity_one_and_zero() {
    let mut one = LruCache::new(1);
    assert_eq!(one.put("a", 1), None);
    assert_eq!(one.put("b", 2), ev("a", 1));
    assert_eq!(one.get("b"), Some(2));

    let mut zero = LruCache::new(0);
    assert_eq!(zero.put("a", 1), ev("a", 1));
    assert_eq!(zero.len(), 0);
    assert_eq!(zero.get("a"), None);
}
