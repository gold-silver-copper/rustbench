use task::cart::{self, LineItem};
use task::checkout;

fn items() -> Vec<LineItem> {
    vec![LineItem::new("notebook", 450, 3), LineItem::new("stapler", 1299, 1)]
}

#[test]
fn no_discount_is_unchanged() {
    assert_eq!(cart::total(&items(), 0), 2649);
    assert_eq!(cart::total(&[], 0), 0);
}

#[test]
fn discount_rounds_down() {
    assert_eq!(cart::total(&[LineItem::new("x", 999, 1)], 10), 899);
    assert_eq!(cart::total(&items(), 25), 1986);
    assert_eq!(cart::total(&items(), 100), 0);
}

#[test]
fn checkout_still_charges_full_price() {
    assert_eq!(checkout::shipping(&items()), 499);
    assert_eq!(checkout::amount_due(&items()), 3148);
    assert!(checkout::summary(&items()).contains("due: 3148"));
}
