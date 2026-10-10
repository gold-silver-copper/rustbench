/// Returns the second largest *distinct* value in `values`: the largest
/// value strictly smaller than the maximum. Returns `None` if `values` has
/// fewer than two distinct values.
///
/// ```
/// assert_eq!(task::second_largest(&[4, 9, 9, 2]), Some(4));
/// assert_eq!(task::second_largest(&[5, 5]), None);
/// ```
pub fn second_largest(values: &[i64]) -> Option<i64> {
    let max = values.iter().copied().max()?;
    values.iter().copied().filter(|&v| v < max).max()
}
