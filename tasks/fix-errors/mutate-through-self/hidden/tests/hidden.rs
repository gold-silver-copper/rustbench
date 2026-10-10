#[test]
fn counts() {
    let mut c = task::Counter::new("x");
    assert_eq!(c.visit(), 1);
    assert_eq!(c.visit(), 2);
    assert_eq!(c.visits, 2);
}
