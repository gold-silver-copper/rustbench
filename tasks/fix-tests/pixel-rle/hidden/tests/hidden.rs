use task::{decode, encode};

#[test]
fn short_rows() {
    assert_eq!(encode(""), "");
    assert_eq!(encode("x"), "1x");
    assert_eq!(encode("............"), "12.");
    assert_eq!(encode("##..#"), "2#2.1#");
}

#[test]
fn round_trip() {
    for row in ["..##...", "#", "", "abcabc", "zzzzzzzzzzzzzzzzzzzzzzzzz."] {
        assert_eq!(decode(&encode(row)).as_deref(), Some(row));
    }
}
