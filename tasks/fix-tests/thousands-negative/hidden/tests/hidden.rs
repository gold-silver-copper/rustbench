use task::group_thousands;

#[test]
fn positives() {
    assert_eq!(group_thousands(0), "0");
    assert_eq!(group_thousands(999), "999");
    assert_eq!(group_thousands(1000), "1,000");
    assert_eq!(group_thousands(1234567), "1,234,567");
}

#[test]
fn negatives() {
    assert_eq!(group_thousands(-5), "-5");
    assert_eq!(group_thousands(-999), "-999");
    assert_eq!(group_thousands(-1000), "-1,000");
    assert_eq!(group_thousands(-987654321), "-987,654,321");
}
