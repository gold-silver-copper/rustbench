use task::{from_roman, to_roman};

#[test]
fn writes() {
    assert_eq!(to_roman(1).as_deref(), Some("I"));
    assert_eq!(to_roman(4).as_deref(), Some("IV"));
    assert_eq!(to_roman(49).as_deref(), Some("XLIX"));
    assert_eq!(to_roman(444).as_deref(), Some("CDXLIV"));
    assert_eq!(to_roman(2024).as_deref(), Some("MMXXIV"));
    assert_eq!(to_roman(3999).as_deref(), Some("MMMCMXCIX"));
    assert_eq!(to_roman(0), None);
    assert_eq!(to_roman(4000), None);
}

#[test]
fn round_trips() {
    for n in 1..=3999 {
        let s = to_roman(n).unwrap();
        assert_eq!(from_roman(&s), Some(n), "{s}");
    }
}

#[test]
fn rejects_non_standard() {
    for s in ["", "IIII", "VV", "IC", "IL", "XM", "MMMM", "iv", "XIIX", "LL", "DD", "IVI", "CMC", " X", "X ", "A", "VX"] {
        assert_eq!(from_roman(s), None, "{s:?}");
    }
}
