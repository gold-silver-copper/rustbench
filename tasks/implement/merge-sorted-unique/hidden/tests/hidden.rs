use task::merge_sorted_unique;

#[test]
fn merges() {
    assert_eq!(merge_sorted_unique(&[1, 1, 1], &[1, 1]), vec![1]);
    assert_eq!(merge_sorted_unique(&[-5, 0, 10], &[-7, 0, 0, 20]), vec![-7, -5, 0, 10, 20]);
    assert_eq!(merge_sorted_unique(&[4, 5, 6], &[1, 2, 3]), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(merge_sorted_unique(&[i32::MIN, i32::MAX], &[i32::MAX]), vec![i32::MIN, i32::MAX]);
}

#[test]
fn empty_sides() {
    assert_eq!(merge_sorted_unique(&[], &[]), Vec::<i32>::new());
    assert_eq!(merge_sorted_unique(&[2, 2, 3], &[]), vec![2, 3]);
    assert_eq!(merge_sorted_unique(&[], &[9, 9]), vec![9]);
}
