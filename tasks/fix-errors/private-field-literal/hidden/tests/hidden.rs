#[test]
fn starter() {
    let mut w = task::starter_wallet("mo");
    assert_eq!(w.owner(), "mo");
    assert_eq!(w.coins(), 100);
    assert!(w.spend(30));
    assert!(!w.spend(80));
    assert_eq!(w.coins(), 70);
}
