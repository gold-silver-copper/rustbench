use task::sum_even;

#[test]
fn mixed() {
    assert_eq!(sum_even(&[1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(sum_even(&[-4, -3, -2, 7]), -6);
}

#[test]
fn nothing_even() {
    assert_eq!(sum_even(&[]), 0);
    assert_eq!(sum_even(&[1, -1, 3, -5]), 0);
}
