#[test]
fn badges() {
    let (b, n) = task::badges(vec!["Ann".into(), "Raj".into()]);
    assert_eq!(b, vec!["HELLO, my name is Ann", "HELLO, my name is Raj"]);
    assert_eq!(n, 2);
}
