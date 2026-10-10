use task::second_largest;

#[test]
fn finds() {
    assert_eq!(second_largest(&[1, 2]), Some(1));
    assert_eq!(second_largest(&[2, 1, 2, 2]), Some(1));
    assert_eq!(second_largest(&[-3, -1, -2, -1]), Some(-2));
    assert_eq!(second_largest(&[i64::MIN, i64::MAX, i64::MAX]), Some(i64::MIN));
}

#[test]
fn too_few() {
    assert_eq!(second_largest(&[]), None);
    assert_eq!(second_largest(&[7]), None);
    assert_eq!(second_largest(&[3, 3, 3]), None);
}
