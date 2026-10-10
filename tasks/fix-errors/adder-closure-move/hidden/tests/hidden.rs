#[test]
fn taxes() {
    let vat = task::with_tax(20);
    assert_eq!(vat(1000), 1200);
    assert_eq!(vat(99), 118);
    assert_eq!(task::with_tax(0)(5), 5);
}
