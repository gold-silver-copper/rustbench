use task::{frost_hours, frost_risk};

#[test]
fn limit_counts() {
    assert!(frost_risk(0.0));
    assert!(frost_risk(-5.0));
    assert!(!frost_risk(0.1));
}

#[test]
fn whole_night() {
    assert_eq!(frost_hours(&[0.0, 0.0, 1.0, -0.5]), vec![0, 1, 3]);
    assert_eq!(frost_hours(&[]), Vec::<usize>::new());
}
