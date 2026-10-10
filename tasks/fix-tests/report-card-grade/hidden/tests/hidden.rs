use task::grade;

#[test]
fn boundaries() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(89), 'B');
    assert_eq!(grade(80), 'B');
    assert_eq!(grade(70), 'C');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(59), 'F');
    assert_eq!(grade(0), 'F');
}
