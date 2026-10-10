use task::merge_intervals;

#[test]
fn merges() {
    assert_eq!(merge_intervals(&[]), Ok(vec![]));
    assert_eq!(merge_intervals(&[(3, 3)]), Ok(vec![(3, 3)]));
    assert_eq!(merge_intervals(&[(1, 10), (2, 3), (4, 5)]), Ok(vec![(1, 10)]));
    assert_eq!(merge_intervals(&[(1, 2), (4, 5)]), Ok(vec![(1, 2), (4, 5)]));
    assert_eq!(merge_intervals(&[(4, 5), (1, 3)]), Ok(vec![(1, 5)]));
    assert_eq!(merge_intervals(&[(-5, -1), (0, 0), (2, 2), (2, 2)]), Ok(vec![(-5, 0), (2, 2)]));
    assert_eq!(merge_intervals(&[(0, 100), (5, 6), (100, 200), (202, 202)]), Ok(vec![(0, 200), (202, 202)]));
}

#[test]
fn extremes() {
    assert_eq!(
        merge_intervals(&[(i64::MAX, i64::MAX), (i64::MIN, i64::MIN), (i64::MAX - 1, i64::MAX - 1)]),
        Ok(vec![(i64::MIN, i64::MIN), (i64::MAX - 1, i64::MAX)])
    );
    assert_eq!(merge_intervals(&[(i64::MIN, 0), (1, i64::MAX)]), Ok(vec![(i64::MIN, i64::MAX)]));
    assert_eq!(merge_intervals(&[(0, i64::MAX), (5, 5), (i64::MAX, i64::MAX)]), Ok(vec![(0, i64::MAX)]));
}

#[test]
fn invalid() {
    assert_eq!(merge_intervals(&[(2, 1)]), Err(0));
    assert_eq!(merge_intervals(&[(0, 0), (1, 1), (9, 8), (7, 6)]), Err(2));
}
