use task::squeeze;

#[test]
fn squeezes() {
    assert_eq!(squeeze("a    b", " "), "a b");
    assert_eq!(squeeze("xxyyxx", "x"), "xyyx");
    assert_eq!(squeeze("--- ---", "-"), "- -");
    assert_eq!(squeeze("ééé!!", "é"), "é!!");
    assert_eq!(squeeze("a-a-a", "-a"), "a-a-a");
}

#[test]
fn nothing_to_do() {
    assert_eq!(squeeze("", "abc"), "");
    assert_eq!(squeeze("aabb", ""), "aabb");
    assert_eq!(squeeze("aabb", "z"), "aabb");
}
