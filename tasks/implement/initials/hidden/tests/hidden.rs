use task::initials;

#[test]
fn letters() {
    assert_eq!(initials("grace brewster murray hopper"), "GBMH");
    assert_eq!(initials("\témile\nzola "), "ÉZ");
    assert_eq!(initials("ßeta x"), "SSX");
}

#[test]
fn skipped_words() {
    assert_eq!(initials(""), "");
    assert_eq!(initials("  "), "");
    assert_eq!(initials("42 _x -y (z) w"), "W");
}
