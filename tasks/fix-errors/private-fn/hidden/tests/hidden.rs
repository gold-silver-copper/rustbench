#[test]
fn lines() {
    assert_eq!(task::forecast_line("Mon", 20.0), "Mon: 68F");
    assert_eq!(task::forecast_line("Tue", -40.0), "Tue: -40F");
    assert!(task::temperature::is_freezing(0.0));
}
