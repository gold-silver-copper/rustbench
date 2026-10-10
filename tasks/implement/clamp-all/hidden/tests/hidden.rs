use task::clamp_all;

#[test]
fn clamps() {
    assert_eq!(clamp_all(&[i32::MIN, -1, 0, 7, 8, i32::MAX], -1, 7), Some(vec![-1, -1, 0, 7, 7, 7]));
    assert_eq!(clamp_all(&[1, 2, 3], 2, 2), Some(vec![2, 2, 2]));
    assert_eq!(clamp_all(&[], 0, 1), Some(vec![]));
}

#[test]
fn backwards_bounds() {
    assert_eq!(clamp_all(&[], 3, 2), None);
    assert_eq!(clamp_all(&[1, 2], 10, -10), None);
}
