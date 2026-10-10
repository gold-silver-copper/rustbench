#[test]
fn lists() {
    assert_eq!(task::numbered(&["apples", "pears"]), "1. apples\n2. pears\n");
    assert_eq!(task::numbered(&[10, 20]), "1. 10\n2. 20\n");
    assert_eq!(task::numbered::<u8>(&[]), "");
}
