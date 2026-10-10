/// Merges two slices, each sorted in ascending order, into one strictly
/// ascending vector.
///
/// Every duplicate is removed, whether it repeats within one slice or
/// appears in both. Either slice may be empty.
///
/// ```
/// assert_eq!(task::merge_sorted_unique(&[1, 3, 3, 7], &[2, 3, 8]), vec![1, 2, 3, 7, 8]);
/// ```
pub fn merge_sorted_unique(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}
