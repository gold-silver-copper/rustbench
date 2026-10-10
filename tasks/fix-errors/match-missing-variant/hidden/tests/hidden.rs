use task::{fee, Shipping};

#[test]
fn fees() {
    assert_eq!(fee(Shipping::Standard), 499);
    assert_eq!(fee(Shipping::Express), 999);
    assert_eq!(fee(Shipping::Overnight), 1999);
    assert_eq!(fee(Shipping::Pickup), 0);
}
