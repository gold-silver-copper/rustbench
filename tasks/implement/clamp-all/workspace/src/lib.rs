/// Clamps every value into the inclusive range `lo..=hi`, keeping the order.
///
/// Values below `lo` become `lo`, values above `hi` become `hi`, and the rest
/// are unchanged. `lo == hi` is allowed and turns every value into it.
/// Returns `None` when `lo > hi`, even if `values` is empty.
///
/// ```
/// assert_eq!(task::clamp_all(&[-5, 3, 12], 0, 10), Some(vec![0, 3, 10]));
/// assert_eq!(task::clamp_all(&[1], 5, 4), None);
/// ```
pub fn clamp_all(values: &[i32], lo: i32, hi: i32) -> Option<Vec<i32>> {
    todo!()
}
