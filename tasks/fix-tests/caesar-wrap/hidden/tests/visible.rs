use task::encrypt;

#[test]
fn wraps_past_z() {
    assert_eq!(encrypt("xyz", 3), "abc");
}
