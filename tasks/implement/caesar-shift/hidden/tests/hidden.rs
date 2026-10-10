use task::caesar_shift;

#[test]
fn wraps() {
    assert_eq!(caesar_shift("xyz XYZ", 3), "abc ABC");
    assert_eq!(caesar_shift("abc", 26), "abc");
    assert_eq!(caesar_shift("abc", 27), "bcd");
    assert_eq!(caesar_shift("abc", -27), "zab");
    assert_eq!(caesar_shift("Rust", 26 * 1000 + 13), "Ehfg");
}

#[test]
fn extremes_and_others() {
    let s = "The Quick Brown Fox";
    assert_eq!(caesar_shift(&caesar_shift(s, i32::MAX), -(i32::MAX % 26)), s);
    assert_eq!(caesar_shift("a", i32::MIN), caesar_shift("a", i32::MIN.rem_euclid(26)));
    assert_eq!(caesar_shift("é1 ü-!", 5), "é1 ü-!");
    assert_eq!(caesar_shift("", 4), "");
}
