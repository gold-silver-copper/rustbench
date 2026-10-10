use task::window_sums;

#[test]
fn windows() {
    assert_eq!(window_sums(&[1, 2, 3, 4, 5, 6, 7], 3, 2), Some(vec![6, 12, 18]));
    assert_eq!(window_sums(&[1, 2, 3, 4, 5, 6], 3, 2), Some(vec![6, 12]));
    assert_eq!(window_sums(&[5, -5, 5], 1, 1), Some(vec![5, -5, 5]));
    assert_eq!(window_sums(&[1, 2, 3], 3, 5), Some(vec![6]));
    assert_eq!(window_sums(&[1, 2, 3], 1, 10), Some(vec![1]));
}

#[test]
fn no_overflow() {
    assert_eq!(window_sums(&[i32::MAX, i32::MAX, i32::MAX], 3, 1), Some(vec![3 * i32::MAX as i64]));
}

#[test]
fn edges() {
    assert_eq!(window_sums(&[1, 2], 0, 1), None);
    assert_eq!(window_sums(&[1, 2], 1, 0), None);
    assert_eq!(window_sums(&[], 0, 0), None);
    assert_eq!(window_sums(&[1, 2], 3, 1), Some(vec![]));
    assert_eq!(window_sums(&[], 1, 1), Some(vec![]));
    assert_eq!(window_sums(&[1, 2], usize::MAX, 1), Some(vec![]));
}
