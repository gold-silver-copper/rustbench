/// Returns the sum of the even numbers in `values`.
///
/// Negative even numbers count too (`-4` is even, and so is `0`). An empty
/// slice, or one with no even numbers, sums to `0`.
///
/// ```
/// assert_eq!(task::sum_even(&[1, 2, 3, 4]), 6);
/// assert_eq!(task::sum_even(&[-4, 3, 0]), -4);
/// ```
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}
