use task::encode;

#[test]
fn encodes_a_row() {
    assert_eq!(encode("aaab"), "3a1b");
}
