use task::{Discount, Flat, Percent};

#[test]
fn applies_in_order() {
    let ds: Vec<&dyn Discount> = vec![&Percent(10), &Flat(50)];
    assert_eq!(task::checkout(1000, &ds), 850);
    let ds: Vec<&dyn Discount> = vec![&Flat(2000)];
    assert_eq!(task::checkout(1000, &ds), 0);
}
