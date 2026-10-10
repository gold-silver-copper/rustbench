use task::split;

#[test]
fn nobody_at_the_table() {
    assert!(split(1000, 0).is_empty());
}
