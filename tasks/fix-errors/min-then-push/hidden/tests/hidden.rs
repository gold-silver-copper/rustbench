#[test]
fn laps() {
    let mut l = Vec::new();
    assert_eq!(task::add_lap(&mut l, 70), 70);
    assert_eq!(task::add_lap(&mut l, 65), 65);
    assert_eq!(task::add_lap(&mut l, 80), 65);
    assert_eq!(l, vec![70, 65, 80]);
}
