#[test]
fn summary() {
    assert_eq!(task::summary(&[500, 250, 500]), "3 items, 1250 cents total, 416 cents average");
    assert_eq!(task::summary(&[]), "0 items, 0 cents total, 0 cents average");
}
