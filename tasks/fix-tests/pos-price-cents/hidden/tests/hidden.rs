use task::{to_cents, total_cents};

#[test]
fn rounds_to_nearest() {
    assert_eq!(to_cents(19.99), 1999);
    assert_eq!(to_cents(4.567), 457);
    assert_eq!(to_cents(4.561), 456);
    assert_eq!(to_cents(-0.29), -29);
    assert_eq!(to_cents(3.0), 300);
}

#[test]
fn totals() {
    assert_eq!(total_cents(&[19.99, 0.29, 1.10]), 2138);
    assert_eq!(total_cents(&[]), 0);
}
