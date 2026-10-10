use task::digit_sum;

#[test]
fn sums() {
    assert_eq!(digit_sum(0), 0);
    assert_eq!(digit_sum(7), 7);
    assert_eq!(digit_sum(1000), 1);
    assert_eq!(digit_sum(999_999), 54);
    assert_eq!(digit_sum(u64::MAX), 87);
}
