use task::{days_in_month, days_in_year, is_leap};

#[test]
fn leap_rules() {
    assert!(is_leap(2000));
    assert!(is_leap(1996));
    assert!(!is_leap(1900));
    assert!(!is_leap(2023));
    assert!(!is_leap(2100));
}

#[test]
fn year_and_month_lengths() {
    assert_eq!(days_in_year(1996), 366);
    assert_eq!(days_in_year(2001), 365);
    assert_eq!(days_in_month(2000, 2), Some(29));
    assert_eq!(days_in_month(1900, 2), Some(28));
    assert_eq!(days_in_month(2023, 13), None);
}
