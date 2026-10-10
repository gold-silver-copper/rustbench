#[test]
fn largest() {
    assert_eq!(task::largest(&[3, 9, 2]), Some(9));
    assert_eq!(task::largest(&[-1.5, -0.5]), Some(-0.5));
    assert_eq!(task::largest(&['a', 'z', 'q']), Some('z'));
    assert_eq!(task::largest::<u8>(&[]), None);
}
