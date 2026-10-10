use task::{can_go, next, Light};

#[test]
fn cycle() {
    assert!(can_go(Light::Green));
    assert!(!can_go(Light::Red));
    assert!(!can_go(Light::Yellow));
    assert!(can_go(next(Light::Red)));
    assert!(!can_go(next(Light::Green)));
}
