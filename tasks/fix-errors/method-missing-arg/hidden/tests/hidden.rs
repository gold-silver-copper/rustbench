#[test]
fn receives() {
    let mut s = task::Stock::default();
    s.add("bolt", 1);
    s.receive(&["bolt", "nut"], 10);
    assert_eq!(s.quantity("bolt"), 11);
    assert_eq!(s.quantity("nut"), 10);
    assert_eq!(s.quantity("gear"), 0);
}
