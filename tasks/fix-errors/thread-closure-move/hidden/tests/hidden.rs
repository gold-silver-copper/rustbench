#[test]
fn sums_chunks() {
    assert_eq!(task::chunk_sums(vec![1, 2, 3, 4, 5], 2), vec![3, 7, 5]);
    assert_eq!(task::chunk_sums(vec![], 3), Vec::<u64>::new());
}
