#[test]
fn curves() {
    let mut s = [50, 95, 100];
    task::apply_curve(&mut s, 10);
    assert_eq!(s, [60, 100, 100]);
}
