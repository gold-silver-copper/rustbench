use task::group_thousands;

#[test]
fn negative_amounts() {
    assert_eq!(group_thousands(-123456), "-123,456");
}
