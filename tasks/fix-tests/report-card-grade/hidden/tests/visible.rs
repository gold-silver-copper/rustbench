use task::report;

#[test]
fn ninety_is_an_a() {
    assert_eq!(
        report(&[("ana", 90), ("ben", 85)]),
        vec![("ana".to_string(), 'A'), ("ben".to_string(), 'B')]
    );
}
