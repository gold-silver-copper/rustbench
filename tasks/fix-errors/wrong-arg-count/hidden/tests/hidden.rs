#[test]
fn totals() {
    assert_eq!(task::final_price(1000, 10, 500), 1400);
    assert_eq!(task::cart_total(&[1000, 2000], 10, 500), 3700);
    assert_eq!(task::cart_total(&[], 50, 500), 0);
}
