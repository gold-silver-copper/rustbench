use task::to_cents;

#[test]
fn typed_price_to_cents() {
    assert_eq!(to_cents(0.29), 29);
}
