#[test]
fn sizes() {
    assert_eq!(task::parse_size("640x480").ok(), Some((640, 480)));
    assert_eq!(task::parse_size(" 3 x 4 ").ok(), Some((3, 4)));
    assert!(task::parse_size("640480").is_err());
    assert!(task::parse_size("ax4").is_err());
    assert!(task::parse_size("4x-1").is_err());
}
