use task::days_in_month;

#[test]
fn february_2024_has_29_days() {
    assert_eq!(days_in_month(2024, 2), Some(29));
}
