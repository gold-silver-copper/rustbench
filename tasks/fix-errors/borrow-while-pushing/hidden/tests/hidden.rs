#[test]
fn compares_with_first() {
    let mut r = vec![20.0, 21.0];
    assert_eq!(task::add_reading(&mut r, 23.5).as_deref(), Some("first 20.0, latest 23.5, change +3.5"));
    assert_eq!(r, vec![20.0, 21.0, 23.5]);
}

#[test]
fn empty_log() {
    let mut r: Vec<f64> = Vec::new();
    assert_eq!(task::add_reading(&mut r, 1.0), None);
    assert!(r.is_empty());
}
