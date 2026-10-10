/// Returns the sums of the windows of `size` consecutive elements of
/// `values` that start at index `0`, `step`, `2 * step`, and so on.
///
/// Only complete windows count: a window that would run past the end of
/// `values` is left out, so a slice shorter than `size` gives an empty
/// vector. Returns `None` if `size` or `step` is 0. Sums are computed in
/// `i64`.
///
/// ```
/// assert_eq!(task::window_sums(&[1, 2, 3, 4, 5], 2, 1), Some(vec![3, 5, 7, 9]));
/// assert_eq!(task::window_sums(&[1, 2, 3, 4, 5], 2, 2), Some(vec![3, 7]));
/// ```
pub fn window_sums(values: &[i32], size: usize, step: usize) -> Option<Vec<i64>> {
    if size == 0 || step == 0 {
        return None;
    }
    let Some(last) = values.len().checked_sub(size) else {
        return Some(Vec::new());
    };
    Some(
        (0..=last)
            .step_by(step)
            .map(|s| values.iter().skip(s).take(size).map(|&v| i64::from(v)).sum())
            .collect(),
    )
}
